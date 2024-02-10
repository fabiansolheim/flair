use std::{fs, path::PathBuf};

use crate::utils::{create_db_client, get_config, init_migration_table};

pub fn status() {
    let mut client = create_db_client();
    let _ = init_migration_table(&mut client);
    let config = get_config();

    match std::fs::read_dir(config.migrations_path.as_path()) {
        Ok(migrations) => {
            if migrations.count() == 0 {
                println!("No migration files found");
                return;
            }
        }
        Err(_) => {
            println!("\x1b[31mError reading migration directory\x1b[0m");
            println!("Run `flair init` to create a migration directory");
            return;
        }
    };

    println!("\n\x1b[34mStatus of your migrations:\x1b[0m");

    let mut files_with_timestamps: Vec<(PathBuf, i64)> = Vec::new();

    for entry in fs::read_dir(config.migrations_path.as_path()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if let Ok(version_id) = file_name.split('_').next().unwrap().parse::<i64>() {
                files_with_timestamps.push((path.clone(), version_id));
            }
        }
    }

    files_with_timestamps.sort_unstable_by_key(|&(_, timestamp)| timestamp);

    for (path, version_id) in files_with_timestamps {
        let mut is_applied = false;
        let query = format!(
            "SELECT * FROM flair_migration WHERE version_id = {}",
            version_id
        );

        for _row in client.query(query.as_str(), &[]).unwrap() {
            is_applied = true;
        }

        let applied = if is_applied {
            "\x1b[32mapplied\x1b[0m"
        } else {
            "\x1b[31mnot applied\x1b[0m"
        };

        println!("  {} - {}", &path.display(), applied);
    }

    println!("\n \x1b[34mUse `flair up` to apply migrations\x1b[0m");
    println!(" \x1b[34mUse `flair down` to revert migrations\x1b[0m");
}
