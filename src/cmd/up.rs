use std::{fs, path::PathBuf};

use crate::utils::{create_db_client, init_migration_table, parse_migration_file};

pub fn up() {
    let mut client = create_db_client();
    let _ = init_migration_table(&mut client);

    let mut files_with_timestamps: Vec<(PathBuf, i64)> = Vec::new();

    for entry in fs::read_dir("migrations").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if let Ok(version_id) = file_name.split('_').next().unwrap().parse::<i64>() {
                files_with_timestamps.push((path.clone(), version_id));
            }
        }
    }

    files_with_timestamps.sort_by_key(|&(_, timestamp)| timestamp);

    for (path, version_id) in files_with_timestamps {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let mut is_applied = false;
        let query = format!(
            "SELECT * FROM flair_migration WHERE version_id = {}",
            version_id
        );
        for _row in client.query(query.as_str(), &[]).unwrap() {
            is_applied = true;
        }

        if !is_applied {
            let content = match parse_migration_file(&path) {
                Ok(content) => content.up,
                Err(e) => {
                    println!("Error reading file: {}", e);
                    continue;
                }
            };

            client.batch_execute(&content).unwrap();
            let insert_query = format!(
                "INSERT INTO flair_migration (version_id, name, is_applied, timestamp) VALUES ({}, '{}', true, now())",
                version_id, file_name
            );
            client.execute(insert_query.as_str(), &[]).unwrap();

            println!("\x1b[32mApplied {}\x1b[0m", file_name);
        }
    }
}
