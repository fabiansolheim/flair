use std::{env::current_dir, io::Write};

use crate::utils::get_timestamp;

const FILE_TEMPLATE: &str = include_str!("../assets/file_template.txt");

pub fn create(name: String) {
    if name.is_empty() {
        println!("\x1b[31mName is empty\x1b[0m");
        return;
    }

    let file_name = format!("{}_{}.sql", get_timestamp(), name);

    if !std::path::Path::new("migrations").exists() {
        println!("\x1b[31mError: Cannot find migration directory. Run `flair init` to create a migration directory\x1b[0m");
        return;
    }
    let path = std::path::Path::new(&current_dir().unwrap())
        .join("migrations")
        .join(file_name);

    if std::path::Path::new(&path).exists() {
        println!("\x1b[31m{} already exists\x1b[0m", name);
        return;
    }
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(FILE_TEMPLATE.as_bytes()).unwrap();
    println!("\x1b[32mCreated {}\x1b[0m", name);
}
