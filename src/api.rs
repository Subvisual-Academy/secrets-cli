use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use const_env::from_env;

pub struct APIClient {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateSecret {
    secret: String,
    expiry: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Room {
    room_id: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Secret {
    secret: String
}

#[from_env("API_URL")]
const API_URL: &'static str = "";

impl APIClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn get_secret(&self, room_id: String) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/secrets/{}", API_URL, room_id);
        let response: Secret = self.client.get(url).send().await?.json().await?;

        Ok(response.secret)
    }

    pub async fn submit_secret(
        &self,
        secret: String,
        expiry: u64,
    ) -> Result<String, Box<dyn Error>> {
        let params = CreateSecret { secret, expiry };
        let url = format!("{}/api/secrets", API_URL);
        
        let response: Room = self
            .client
            .post(url)
            .json(&params)
            .send()
            .await?
            .json()
            .await?;
         
        Ok(response.room_id)
    }
}
