use reqwest::Client;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    key: String,
    localized_name: String,
    country: Country,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.localized_name, self.country.id)
    }
}

#[derive(Deserialize, Debug)]
pub struct Country {
    #[serde(alias = "ID")]
    pub id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Forecast {
    pub headline: Headline,
}

#[derive(Deserialize, Debug)]
pub struct Headline {
    #[serde(alias = "Text")]
    pub overview: String,
}

#[derive(Debug)]
pub struct CouldNotFindLocation {
    place: String,
}

impl Display for CouldNotFindLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find location '{}'", self.place)
    }
}

impl std::error::Error for CouldNotFindLocation {}

pub async fn get_forecast(
    place: &str,
    api_key: &str,
    client: &Client,
) -> Result<(Location, Forecast), Box<dyn std::error::Error>> {
    // Endpoints we will use
    const LOCATION_REQUEST: &str = "http://dataservice.accuweather.com/locations/v1/cities/search";
    const DAY_REQUEST: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/1day/";

    // The URL to call combined with our API_KEY and the place (via the q search parameter)
    let url = format!("{}?apikey={}&q={}", LOCATION_REQUEST, api_key, place);
    // Make the request we will call
    let request = client.get(url).build().unwrap();
    // Execute the request and await a JSON result that will be converted to a
    // vector of locations
    let resp = client
        .execute(request)
        .await?
        .json::<Vec<Location>>()
        .await?;

    // Get the first location. If empty respond with the above declared
    // `CouldNotFindLocation` error type
    let first_location = resp
        .into_iter()
        .next()
        .ok_or_else(|| CouldNotFindLocation {
            place: place.to_owned(),
        })?;

    // Now have the location combine the key/identifier with the URL
    let url = format!("{}{}?apikey={}", DAY_REQUEST, first_location.key, api_key);

    let request = client.get(url).build().unwrap();
    let forecast = client.execute(request).await?.json::<Forecast>().await?;

    // Combine the location with the forecast
    Ok((first_location, forecast))
}
