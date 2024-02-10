use super::create::create;

pub fn init(path: String) {
    match std::fs::create_dir_all(std::path::Path::new(&path)) {
        Ok(_) => {
            println!(
                "\x1b[32mSuccesfully created migrations directory at {}\x1b[0m",
                path
            );
        }
        Err(e) => {
            println!("\x1b[31mError creating migrations directory: {}\x1b[0m", e);
            return;
        }
    }

    match std::fs::File::create("flair.toml") {
        Ok(mut file) => {
            println!("\x1b[32mSuccesfully created flair.toml file to keep track of where you store your migrations \x1b[0m");
            let toml = format!("[migrations]\npath = \"{}\"", path);
            std::io::Write::write_all(&mut file, toml.as_bytes()).unwrap();
        }
        Err(e) => {
            println!("\x1b[31mError creating flair.toml file: {}\x1b[0m", e);
            return;
        }
    }

    create("initial_migration".to_string());
}
