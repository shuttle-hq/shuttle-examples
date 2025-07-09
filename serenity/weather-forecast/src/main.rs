mod weather;

use anyhow::Context as _;
use serenity::all::{GuildId, Interaction};
use serenity::async_trait;
use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use tracing::info;

struct Bot {
    weather_api_key: String,
    client: reqwest::Client,
    discord_guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands = vec![
            CreateCommand::new("hello").description("Say hello"),
            CreateCommand::new("weather")
                .description("Display the weather")
                .add_option(
                    CreateCommandOption::new(
                        serenity::all::CommandOptionType::String,
                        "place",
                        "City to lookup forecast",
                    )
                    .required(true),
                ),
        ];

        let commands = &self
            .discord_guild_id
            .set_commands(&ctx.http, commands)
            .await
            .unwrap();

        info!("Registered commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "weather" => {
                    let argument = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "place")
                        .cloned();

                    let value = argument.unwrap().value;
                    let place = value.as_str().unwrap();

                    let result =
                        weather::get_forecast(place, &self.weather_api_key, &self.client).await;

                    match result {
                        Ok((location, forecast)) => {
                            format!("Forecast: {} in {}", forecast.headline.overview, location)
                        }
                        Err(err) => {
                            format!("Err: {err}")
                        }
                    }
                }
                command => unreachable!("Unknown command: {}", command),
            };

            let data = CreateInteractionResponseMessage::new().content(response_content);
            let builder = CreateInteractionResponse::Message(data);

            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let weather_api_key = secret_store
        .get("WEATHER_API_KEY")
        .context("'WEATHER_API_KEY' was not found")?;

    let discord_guild_id = secret_store
        .get("DISCORD_GUILD_ID")
        .context("'DISCORD_GUILD_ID' was not found")?;

    let client = get_client(
        &discord_token,
        &weather_api_key,
        discord_guild_id.parse().unwrap(),
    )
    .await;
    Ok(client.into())
}

pub async fn get_client(
    discord_token: &str,
    weather_api_key: &str,
    discord_guild_id: u64,
) -> Client {
    // Set gateway intents, which decides what events the bot will be notified about.
    // Here we don't need any intents so empty
    let intents = GatewayIntents::empty();

    Client::builder(discord_token, intents)
        .event_handler(Bot {
            weather_api_key: weather_api_key.to_owned(),
            client: reqwest::Client::new(),
            discord_guild_id: GuildId::new(discord_guild_id),
        })
        .await
        .expect("Err creating client")
}
