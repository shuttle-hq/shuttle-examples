use anyhow::Context as _;
use serenity::async_trait;
use serenity::builder::{
    CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::model::application::{CommandDataOptionValue, CommandOptionType, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use sqlx::{Executor, PgPool};
use tracing::{error, info};

mod db;

struct Bot {
    database: PgPool,
    guild_id: String,
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!("Received command interaction: {:#?}", command);

            let user_id: i64 = command.user.id.into();

            let content = match command.data.name.as_str() {
                "todo" => {
                    let command = command.data.options.first().expect("Expected command");

                    match command.name.as_str() {
                        "add" => match &command.value {
                            CommandDataOptionValue::SubCommand(opts) => {
                                let note = opts.first().unwrap().value.as_str().unwrap();
                                db::add(&self.database, note, user_id).await.unwrap()
                            }
                            _ => "Command not implemented".to_string(),
                        },
                        "complete" => match &command.value {
                            CommandDataOptionValue::SubCommand(opts) => {
                                let index = opts.first().unwrap().value.as_i64().unwrap();
                                db::complete(&self.database, &index, user_id)
                                    .await
                                    .unwrap_or_else(|_| {
                                        "Please submit a valid index from your todo list"
                                            .to_string()
                                    })
                            }
                            _ => "Command not implemented".to_string(),
                        },
                        "list" => db::list(&self.database, user_id).await.unwrap(),
                        _ => "Command not implemented".to_string(),
                    }
                }
                _ => "Command not implemented".to_string(),
            };

            if let Err(why) = command
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(content),
                    ),
                )
                .await
            {
                error!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(self.guild_id.parse().unwrap());

        let _ = guild_id
            .set_commands(
                &ctx.http,
                vec![CreateCommand::new("todo")
                    .description("Add, list and complete todos")
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "add",
                            "Add a new todo",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "note",
                                "The todo note to add",
                            )
                            .min_length(2)
                            .max_length(100)
                            .required(true),
                        ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "complete",
                            "The todo to complete",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::Integer,
                                "index",
                                "The index of the todo to complete",
                            )
                            .min_int_value(1)
                            .required(true),
                        ),
                    )
                    .add_option(CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "list",
                        "List your todos",
                    ))],
            )
            .await;
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;
    // Get the guild_id set in `Secrets.toml`
    let guild_id = secret_store
        .get("GUILD_ID")
        .context("'GUILD_ID' was not found")?;

    // Run the schema migration
    pool.execute(include_str!("../schema.sql"))
        .await
        .context("failed to run migrations")?;

    let bot = Bot {
        database: pool,
        guild_id,
    };
    let client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
