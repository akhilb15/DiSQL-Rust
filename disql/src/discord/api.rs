use reqwest::blocking::Client as HttpClient;
use serde_json::json;

pub struct DiscordApiInterface {
    http_client: HttpClient,
}

impl DiscordApiInterface {
    /// Create a new Discord API client
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }
    /// send a message to a channel
    pub fn send_message(&self, channel_id: &str, token: &str, content: &str) -> Result<String, reqwest::Error> {
        let url = format!("https://discord.com/api/channels/{}/messages", channel_id);
        let body = json!({ "content": content });
        // Send the message and get message id
        let response: serde_json::Value = self.http_client
            .post(&url)
            .header("Authorization", format !("Bot {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?
            .json()?;

        // Get the message id
        let message_id = response["id"].as_str().expect("Message id not found").to_string();
        
        Ok(message_id)
    }

    /// Get message by message id
    pub fn get_message(&self, channel_id: &str, token: &str, message_id: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("https://discord.com/api/channels/{}/messages/{}", channel_id, message_id);
        let response: serde_json::Value = self.http_client
            .get(&url)
            .header("Authorization", format!("Bot {}", token))
            .send()?
            .json()?;

        Ok(response) 
    }
}