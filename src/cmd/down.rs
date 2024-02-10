use crate::utils::{create_db_client, init_migration_table, parse_migration_file};

pub fn down() {
    let mut client = create_db_client();
    let _ = init_migration_table(&mut client);

    let applied_migrations_query =
      "SELECT version_id, name FROM flair_migration WHERE is_applied = true ORDER BY version_id DESC LIMIT 1";
    let applied_migrations = client.query(applied_migrations_query, &[]).unwrap();

    if let Some(row) = applied_migrations.iter().next() {
        let version_id: i64 = row.get("version_id");
        let file_name: String = row.get("name");
        let path = std::path::Path::new("migrations").join(file_name);

        if path.exists() {
            let content = match parse_migration_file(&path) {
                Ok(content) => content,
                Err(e) => {
                    println!("\x1b[31mError reading file: {}\x1b[0m", e);
                    return;
                }
            };

            client.batch_execute(&content.down).unwrap();
            let update_query = format!(
                "DELETE FROM flair_migration WHERE version_id = {}",
                version_id
            );
            client.execute(update_query.as_str(), &[]).unwrap();

            println!("\x1b[32mReverted {}\x1b[0m", path.display());
        }
    } else {
        println!("\x1b[34mNo migrations to revert.");
    }
}
