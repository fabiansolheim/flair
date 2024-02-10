use chrono::Local;
use core::panic;
use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;
use std::{fs, io::Error, path::PathBuf};

use crate::Command;

pub struct Config {
    pub conn_str: String,
}

pub fn get_config() -> Config {
    dotenv().ok();
    let conn_str = match env::var("FLAIR_DBSTRING") {
        Ok(val) => val,
        Err(e) => {
            println!(
                "\x1b[31m Couldn't find FLAIR_DBSTRING in environment variables: {}\x1b[0m",
                e.to_string()
            );
            panic!("FLAIR_DBSTRING not found");
        }
    };
    Config {
        conn_str: conn_str.to_string(),
    }
}

pub fn init_migration_table(client: &mut Client) -> Result<(), Error> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS flair_migration (id SERIAL PRIMARY KEY,
            version_id BIGINT, name VARCHAR, is_applied boolean, timestamp TIMESTAMP)",
            &[],
        )
        .unwrap();
    Ok(())
}

pub fn create_db_client() -> Client {
    let config = get_config();
    let client = match Client::connect(&config.conn_str, NoTls) {
        Ok(client) => client,
        Err(e) => {
            println!("Make sure the database is running and the connection string is correct");
            println!("Format: host=localhost user=user password=******* dbname=database");
            panic!("Error connecting to database: {}", e);
        }
    };
    return client;
}

pub fn get_timestamp() -> String {
    let now = Local::now();

    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    return timestamp;
}

pub struct Migration {
    pub up: String,
    pub down: String,
}

pub fn parse_migration_file(path: &PathBuf) -> Result<Migration, Error> {
    let content = fs::read_to_string(path)?;

    let parts = content.split("-- Up --").collect::<Vec<&str>>();
    let after_up = parts
        .get(1)
        .ok_or_else(|| Error::new(std::io::ErrorKind::NotFound, "-- Up -- not found"))?;

    let up_and_down_parts = after_up.split("-- Down --").collect::<Vec<&str>>();
    let up = up_and_down_parts
        .get(0)
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::NotFound,
                "-- Down -- not found before content",
            )
        })?
        .trim();

    let down = up_and_down_parts
        .get(1)
        .ok_or_else(|| {
            Error::new(
                std::io::ErrorKind::NotFound,
                "-- Down -- not found after content",
            )
        })?
        .trim();

    Ok(Migration {
        up: up.to_string(),
        down: down.to_string(),
    })
}

pub fn parse_args(args: &[String]) -> Command {
    if args.len() < 2 {
        return Command::Help;
    }

    match args[1].as_str() {
        "help" => Command::Help,
        "init" => Command::Init,
        "status" => Command::Status,
        "create" => {
            if args.len() < 3 {
                return Command::Unknown("No name provided".to_string());
            }
            Command::Create(args[2].clone())
        }
        "down" => Command::Down,
        "up" => Command::Up,
        _ => Command::Unknown(args[1].clone()),
    }
}
