use std::io::Write;

use crate::utils::{get_config, get_timestamp};

const FILE_TEMPLATE: &str = include_str!("../assets/file_template.txt");

pub fn create(name: String) {
    let config = get_config();
    if name.is_empty() {
        println!("\x1b[31mName is empty\x1b[0m");
        return;
    }

    let file_name = format!("{}_{}.sql", get_timestamp(), name);

    if !std::path::Path::new(config.migrations_path.as_path()).exists() {
        println!("\x1b[31mError: Cannot find migration directory. Run `flair init` to create a migration directory\x1b[0m");
        return;
    }
    let file_path = std::path::Path::new(config.migrations_path.as_path()).join(file_name);

    if std::path::Path::new(&file_path).exists() {
        println!("\x1b[31m{} already exists\x1b[0m", name);
        return;
    }
    let mut file = std::fs::File::create(file_path).unwrap();
    file.write_all(FILE_TEMPLATE.as_bytes()).unwrap();
    println!("\x1b[32mCreated {}\x1b[0m", name);
}
