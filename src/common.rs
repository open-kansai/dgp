use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use serenity::all::{GuildId, GuildInfo, PartialGuild};
use std::error::Error;

use crate::DEFAULT_PATH_AUTH_DATA;

#[derive(Deserialize, Serialize)]
pub struct AuthData {
    /// The token will be stored here, unfortunately in an unencrypted format
    pub token: String,
}

impl Default for AuthData {
    fn default() -> Self {
        Self {
            token: Default::default(),
        }
    }
}

impl AuthData {
    pub fn read_data() -> Result<Self, Box<dyn Error>> {
        let path = Path::new(DEFAULT_PATH_AUTH_DATA);

        if !path.exists() {
            println!("Please log in to your discord account using > DGP Auth");
            return Ok(Self::default());
        }

        let data = fs::read_to_string(path)?;
        let data: AuthData = toml::from_str(&data)?;

        Ok(data)
    }

    pub async fn verify_auth_data(&self) -> bool {
        let http = serenity::http::Http::new(&format!("Bot {}", &self.token));

        match http.get_current_user().await {
            Ok(user) => {
                println!("The token has been verified and login to the account is successful under the username {}", user.name);
                true
            }
            Err(_) => {
                println!("Authorization attempt not successful");
                false
            }
        }
    }

    pub fn delete_auth_data() -> io::Result<()> {
        let path = Path::new(DEFAULT_PATH_AUTH_DATA);
        fs::remove_file(path)
    }

    pub fn write_data(token: impl Into<String>) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(DEFAULT_PATH_AUTH_DATA);

        let token: String = token.into();
        let token = token.trim();

        let data = Self {
            token: token.to_string(),
        };

        fs::write(path, toml::to_string(&data)?)?;

        Ok(data)
    }
}

pub struct DGPFunction {
    http: serenity::http::Http,
}

impl DGPFunction {
    pub fn new(auth_data: &AuthData) -> Self {
        Self {
            http: serenity::http::Http::new(&auth_data.token),
        }
    }

    pub async fn get_guilds(&self) -> Vec<GuildInfo> {
        self.http.get_guilds(None, None).await.unwrap()
    }

    pub async fn get_guild(&self, guild_id: u64) -> PartialGuild {
        self.http.get_guild(GuildId::new(guild_id)).await.unwrap()
    }
}
