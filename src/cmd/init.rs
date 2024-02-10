use super::create::create;

pub fn init() {
    match std::fs::create_dir("migrations") {
        Ok(_) => {
            println!("\x1b[32mSuccesfully created migrations directory \x1b[0m");
            create("initial_migration".to_string());
        }
        Err(e) => println!("\x1b[31mError creating migrations directory: {}\x1b[0m", e),
    }
}
