use reqwest::header::AUTHORIZATION;
use serde::{Deserialize};

const BASE: &str = "https://api.animality.xyz";

#[derive(Deserialize)]
pub struct APIImageResult {
    pub link: String,
}

#[derive(Deserialize)]
pub struct APIFactResult {
    pub fact: String,
}

pub struct Animality<'a> {
    key: &'a str
}

impl<'a> Animality<'a> {
    pub fn new(key: &'a str) -> Self {
        Self {
            key
        }
    }

    pub async fn fetch_random_image(self: &Self, animal: String) -> Result<APIImageResult, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(BASE.to_owned() + "/img/" + &animal)
            .header(AUTHORIZATION, self.key)
            .send()
            .await?
            .json::<APIImageResult>()
            .await?;

        Ok(resp)
    }

    pub async fn fetch_fact(self: &Self, animal: String) -> Result<APIFactResult, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get(BASE.to_owned() + "/fact/" + &animal)
            .header(AUTHORIZATION, self.key)
            .send()
            .await?
            .json::<APIFactResult>()
            .await?;

        Ok(resp)
    }
}