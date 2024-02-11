use std::{
    fs,
    path::{Path, PathBuf},
};

use postgres::Client;

use crate::utils::{create_db_client, get_config, init_migration_table, parse_migration_file};

pub fn up() {
    let mut client = create_db_client();
    let _ = init_migration_table(&mut client);
    let config = get_config();

    let files_with_timestamps = get_migration_files(&config.migrations_path.as_path());

    files_with_timestamps.iter().for_each(|(path, version_id)| {
        if !is_migration_applied(&mut client, path) {
            apply_migration(&mut client, path, version_id);
        }
    });
}

fn get_migration_files(migrations_path: &Path) -> Vec<(PathBuf, i64)> {
    fs::read_dir(migrations_path)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .and_then(|name| name.split('_').next())
                    .and_then(|version| version.parse::<i64>().ok())
                    .map(|version_id| (path.clone(), version_id))
            } else {
                None
            }
        })
        .collect::<Vec<(PathBuf, i64)>>()
}

fn is_migration_applied(client: &mut Client, path: &PathBuf) -> bool {
    let version_id = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .split('_')
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let query = format!(
        "SELECT * FROM flair_migration WHERE version_id = {}",
        version_id
    );

    !client.query(query.as_str(), &[]).unwrap().is_empty()
}

fn apply_migration(client: &mut Client, path: &PathBuf, version_id: &i64) {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    match parse_migration_file(&path) {
        Ok(content) => {
            client.batch_execute(&content.up).unwrap();
            let insert_query = format!(
                "INSERT INTO flair_migration (version_id, name, is_applied, timestamp) VALUES ({}, '{}', true, now())",
                version_id, file_name
            );
            client.execute(insert_query.as_str(), &[]).unwrap();
            println!("\x1b[32mApplied {}\x1b[0m", file_name);
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
