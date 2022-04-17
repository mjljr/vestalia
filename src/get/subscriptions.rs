#[cfg(test)]
use mockito;

use crate::error::Error;
use serde::{Deserialize, Serialize};

/// Handles GET body JSON responses from the Vestaboard API
///
/// For more information on the JSON struct see https://docs.vestaboard.com/methods
#[derive(Debug, Deserialize, Serialize)]
struct Response {
    subscriptions: Vec<Subscription>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Subscription {
    #[serde(rename = "_created")]
    created: String,
    #[serde(rename = "_id")]
    id: String,
    boards: Vec<Boards>,
    icon: Option<String>,
    installation: Installation,
    title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Boards {
    #[serde(rename = "_id")]
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Installation {
    #[serde(rename = "_id")]
    id: String,
    installable: Installable,
}

#[derive(Debug, Deserialize, Serialize)]
struct Installable {
    #[serde(rename = "_id")]
    id: String,
}

async fn all(api_key: &str, api_secret: &str) -> Result<Vec<String>, Error> {
    let request = request(api_key, api_secret).await;
    match request {
        Ok(request) => {
            let mut subscription_ids = Vec::new();
            for subscription in request.subscriptions {
                subscription_ids.push(subscription.id);
            }
            Ok(subscription_ids)
        }
        Err(error) => Err(error),
    }
}

pub async fn first(api_key: &str, api_secret: &str) -> Result<String, Error> {
    match all(api_key, api_secret).await {
        Ok(subscriptions) => Ok(subscriptions[0].to_string()),
        Err(error) => Err(error),
    }
}

async fn request(api_key: &str, api_secret: &str) -> Result<Response, Error> {
    #[cfg(not(test))]
    let url = "https://platform.vestaboard.com";

    #[cfg(test)]
    let url = &mockito::server_url();
    let client = reqwest::Client::new()
        .get(format!("{url}/subscriptions"))
        .header("X-Vestaboard-Api-Key", api_key)
        .header("X-Vestaboard-Api-Secret", api_secret);
    let response = match client.send().await {
        Ok(res) => res,
        Err(error) => return Err(Error::Api { source: error }),
    };
    match response.json::<Response>().await {
        Ok(json) => Ok(json),
        Err(error) => return Err(Error::Api { source: error }),
    }
}
