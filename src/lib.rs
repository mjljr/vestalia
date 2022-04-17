#![crate_name = "vestalia"]

#[cfg(test)]
use mockito;

use error::Error;
use serde::{Deserialize, Serialize};

mod error;
pub mod format;
mod get;
mod validators;

#[derive(Debug, Serialize, Deserialize)]
pub enum Body {
    #[serde(rename = "text")]
    Text(String),
    #[serde(rename = "characters")]
    Characters(Vec<Vec<i32>>),
}

struct Installable {
    api_key: String,
    api_secret: String,
    subscription: Option<String>,
}

/// Handles POST body JSON responses from the Vestaboard API
///
/// For more information on the JSON struct see https://docs.vestaboard.com/methods
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub text: Option<String>,
    pub created: String,
}

/// Generates a Vestaboard instance specific to an
/// [installable](https://docs.vestaboard.com/concepts)
///
/// Any software that is publishing content to a Board is considered
/// an installable and has associated tenant installations with API key pairs
/// and subscription ids
pub struct Vestaboard {
    installable: Installable,
}

impl Vestaboard {
    /// Builds the Vestaboard struct with the required API key pair to access the
    /// Vestaboard API
    ///
    /// For more information on creating API keys, visit
    /// [web.vestaboard.com](https://web.vestaboard.com)
    ///
    /// # Arguments
    ///
    /// * `api_key` - A `String` type that holds the API key for the installable
    /// * `api_secret` - A `String` type that holds the associated API secret for the API key
    ///
    /// # Examples
    /// ```
    /// use vestalia::Vestaboard;
    ///
    /// let client = Vestaboard::new("test".to_string(), "test_secret".to_string());
    /// ```
    pub fn new(api_key: String, api_secret: String) -> Vestaboard {
        let installable = Installable {
            api_key: api_key,
            api_secret: api_secret,
            subscription: None,
        };
        Vestaboard {
            installable: installable,
        }
    }

    /// Allows the configuration of a subscription ID for an installation to post messages to a
    /// Board
    ///
    /// If the subscription ID is known, and the intention is to avoid making a separate GET
    /// requests to determine the subscription ID when creating the Vestaboard instance, a
    /// subscription ID can be passed along during initialization
    ///
    /// # Arguments
    ///
    /// * `sub` - A `String` type that holds the subscription ID for an installation
    ///
    /// # Examples
    /// ```
    /// use vestalia::Vestaboard;
    ///
    /// let client = Vestaboard::new("test".to_string(), "test_secret".to_string())
    ///     .subscription("123456a1-1b2c-1b5d-d234-c123456789ab".to_string());
    /// ```
    pub fn subscription(mut self, sub: String) -> Vestaboard {
        self.installable.subscription = Some(sub);
        self
    }

    async fn request(self, json: Body) -> Result<Post, Error> {
        let sub = match self.installable.subscription {
            None => match crate::get::subscriptions::first(
                &self.installable.api_key,
                &self.installable.api_secret,
            )
            .await
            {
                Ok(subscription) => subscription,
                Err(error) => return Err(error),
            },
            Some(sub) => sub.to_string(),
        };

        let client = reqwest::Client::new()
            .post(build_url(sub))
            .json(&json)
            .header("X-Vestaboard-Api-Key", self.installable.api_key)
            .header("X-Vestaboard-Api-Secret", self.installable.api_secret);
        let response = match client.send().await {
            Ok(response) => response,
            Err(error) => return Err(Error::Api { source: error }),
        };
        match response.json::<Post>().await {
            Ok(response) => Ok(response),
            Err(error) => return Err(Error::Api { source: error }),
        }
    }

    /// Sends a supported 6x22 vector as a 1:1 character mapping to a Board
    ///
    /// Vestaboard has the capability of accepting an array of 132 characters in a
    /// 6x22 format. Each supported character is represented in the Vestaboard
    /// [Character Code Reference](https://docs.vestaboard.com/characters)
    ///
    /// # Arguments
    ///
    /// * `characters` - A `Vec<Vec<i32>>` type, **MUST** be 6x22 or validation will fail
    ///
    /// # Examples
    /// ```ignore
    /// use vestalia::Vestaboard;
    ///
    /// let client = Vestaboard::new("test".to_string(), "test_secret".to_string());
    /// // Fill the board with PoppyRed! (6x22 Vec containing character code 63 in all spaces)
    /// client.characters(vec![vec![63; 22]; 6]).await;
    /// ```
    pub async fn characters(self, characters: Vec<Vec<i32>>) -> Result<Post, Error> {
        if !validators::is_valid_vec(&characters) {
            return Err(Error::VecValidation);
        }
        let json = Body::Characters(characters.into());
        self.request(json).await
    }

    /// Sends a text message to a Board
    ///
    /// Vestaboard's API will handle formatting and messages longer than 132 characters
    /// (or limited line spacing) will be truncated. This string also supports new line breaks
    /// with \n and character codes wrapped in {}. Example {63} is PoppyRed.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice type containing the message to send to the board
    ///
    /// # Examples
    /// ```ignore
    /// use vestalia::Vestaboard;
    ///
    /// let client = Vestaboard::new("test".to_string(), "test_secret".to_string()
    /// let message = "{63}{63}{63}{63}{63}\nHello\nWorld!{63}{63}{63}{63}{63}".to_string();
    /// client.text(message).await;
    /// ```
    pub async fn text(self, text: &str) -> Result<Post, Error> {
        if !validators::is_valid_text(text) {
            return Err(Error::TextValidation);
        }
        let json = Body::Text(text.into());
        self.request(json).await
    }
}

fn build_url(subscription: String) -> String {
    #[cfg(not(test))]
    let url = "https://platform.vestaboard.com";

    #[cfg(test)]
    let url = &mockito::server_url();
    return format!("{url}/subscriptions/{subscription}/message");
}

#[cfg(test)]
mod tests {
    use crate::{Message, Post, Vestaboard};
    use mockito::mock;

    static GET_STRING: &str = r#"
        {
            "subscriptions":
            [{
                "_id":"123456a1-1b2c-1b5d-d234-c123456789ab",
                "_created":"1649630799628",
                "title":null,
                "icon":null,
                "installation": {
                    "_id":"abcdefgh-1234-5678-0000-123456789abc",
                    "installable": {
                        "_id":"01234567-abcd-0123-abcd-0123456789ab"
                    }
                },
                "boards": [{
                    "_id":"abcdefgh-0123-abcd-0123-abcdefghijkl"
                }]
            }]
        }"#;
    static POST_PASSING_STRING: &str = r#"
        {
            "message": {
                "id":"12345678-abcd-0123-aaaa-bbbbccccdddd",
                "text":"Test!",
                "created":"1650168530618"
            }
        }"#;
    #[tokio::test]
    async fn test_vestaboard_no_subscription_valid() {
        let _get_mock = mock("GET", "/subscriptions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(GET_STRING)
            .create();
        let _post_mock = mock(
            "POST",
            "/subscriptions/123456a1-1b2c-1b5d-d234-c123456789ab/message",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(POST_PASSING_STRING)
        .create();
        let post_passing = Post {
            message: Message {
                id: "12345678-abcd-0123-aaaa-bbbbccccdddd".to_string(),
                text: Some("Test!".to_string()),
                created: "1650168530618".to_string(),
            },
        };
        let client = Vestaboard::new("test".to_string(), "test_secret".to_string());
        let send_text = "Test!".to_string();
        let result = client.text(&send_text).await.unwrap();
        assert_eq!(result.message.text, post_passing.message.text);
    }
    #[tokio::test]
    async fn test_vestaboard_with_subscription_valid() {
        let _post_mock = mock(
            "POST",
            "/subscriptions/123456a1-1b2c-1b5d-d234-c123456789ab/message",
        )
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(POST_PASSING_STRING)
        .create();
        let post_passing = Post {
            message: Message {
                id: "12345678-abcd-0123-aaaa-bbbbccccdddd".to_string(),
                text: Some("Test!".to_string()),
                created: "1650168530618".to_string(),
            },
        };
        let client = Vestaboard::new("test".to_string(), "test_secret".to_string())
            .subscription("123456a1-1b2c-1b5d-d234-c123456789ab".to_string());
        let send_text = "Test!".to_string();
        let result = client.text(&send_text).await.unwrap();
        assert_eq!(result.message.text, post_passing.message.text);
    }
    #[tokio::test]
    async fn test_vestaboard_http_error() {
        let _post_mock = mock(
            "POST",
            "/subscriptions/123456a1-1b2c-1b5d-d234-c123456789ab/message",
        )
        .with_status(400)
        .create();
        let client = Vestaboard::new("test".to_string(), "test_secret".to_string())
            .subscription("123456a1-1b2c-1b5d-d234-c123456789ab".to_string());
        let send_text = "Test!".to_string();
        let result = client.text(&send_text).await;
        assert!(result.is_err());
    }
}
