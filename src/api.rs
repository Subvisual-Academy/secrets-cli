use const_env::from_env;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct APIClient {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateSecret {
    secret: String,
    expiry: u64,
}

trait Returnable{
    fn get_return_value(&self) -> &String;
}

#[derive(Debug, Serialize, Deserialize)]
struct Room {
    room_id: String,
}

impl Returnable for Room {
    fn get_return_value(&self) -> &String {
        &self.room_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Secret {
    secret: String,
}

impl Returnable for Secret {
    fn get_return_value(&self) -> &String {
        &self.secret
    }
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
        let response = self.client.get(url).send().await?;
        
        Self::handle_response::<Secret>(response).await 
    }

    pub async fn submit_secret(
        &self,
        secret: String,
        expiry: u64,
    ) -> Result<String, Box<dyn Error>> {
        let params = CreateSecret { secret, expiry };
        let url = format!("{}/api/secrets", API_URL);

        let response = self.client.post(url).json(&params).send().await?;

        Self::handle_response::<Room>(response).await 
    }

    async fn handle_response<T>(response: Response) -> Result<String, Box<dyn Error>>
    where
        T: for<'da> Deserialize<'da> + Returnable,
    {
        let id = response.status();
        if id == 200 {
            let response: T = response.json().await?;

            Ok(response.get_return_value().to_string())
        } else {
            Err(format!("error in request with status {}", id))?
        }
    }
}
