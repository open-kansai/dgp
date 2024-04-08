mod common;

use std::io::{stdin, stdout, Write};

use clap::{Parser, Subcommand};
use common::{AuthData, DGPFunction};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "CLI For DGP", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ShowGuilds {},
    ShowGuildDetail {
        /// Specify Guild id
        guild_id: u64,
    },
    Copy {
        /// Specify Guild id
        guild_id: u64,
    },
    Auth {},
}

pub const DEFAULT_PATH_AUTH_DATA: &'static str = "./.auth_data";

async fn verify_auth_data(auth_data: &AuthData) {
    println!("Started verifying token");
    let is_authorized = auth_data.verify_auth_data().await;

    if is_authorized {
        println!("Succesfully authorized");
        return;
    } else {
        let _ = AuthData::delete_auth_data();
    }
}

async fn verify_auth() -> AuthData {
    let auth_data = AuthData::read_data().unwrap();

    if auth_data.token.is_empty() {
        panic!("Please authorize in account: DGP auth");
    }

    verify_auth_data(&auth_data).await;

    auth_data
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Copy { .. } => {}
        Commands::ShowGuildDetail { guild_id } => {
            let auth_data = verify_auth().await;
            let dgp_function = DGPFunction::new(&auth_data);

            let guild = dgp_function.get_guild(guild_id).await;

            println!("{:#?}", guild);
        }
        Commands::ShowGuilds {} => {
            let auth_data = verify_auth().await;
            let dgp_function = DGPFunction::new(&auth_data);
            let guilds = dgp_function.get_guilds().await;

            if guilds.is_empty() {
                println!("Please add the bot to one or more guilds that you want to copy");
            }

            for guild in guilds {
                println!("Guild: {} - {}", guild.name, guild.id);
            }
        }
        Commands::Auth {} => {
            let mut input = String::new();

            print!("Please enter a token: ");
            stdout().flush().unwrap();

            match stdin().read_line(&mut input) {
                Ok(_) => {
                    let token_regex =
                        Regex::new(r"(mfa\.[\w-]{84}|[\w-]{24}\.[\w-]{6}\.[\w-]{27})").unwrap();

                    if !token_regex.is_match(input.as_str()) {
                        println!("You entered a incorrect token");
                        return;
                    }

                    let auth_data = AuthData::write_data(input);

                    if let Ok(auth_data) = auth_data {
                        verify_auth_data(&auth_data).await;
                        return;
                    }

                    println!("We're got some troubles ;(");
                }
                Err(_) => {}
            }
        }
    }
}
