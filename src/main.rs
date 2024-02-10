mod cmd;
mod utils;

use cmd::{create::create, down::down, init::init, status::status, up::up};
use utils::parse_args;

enum Command {
    Down,
    Init,
    Status,
    Up,
    Help,
    Create(String),
    Unknown(String),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = parse_args(&args);

    match command {
        Command::Up => up(),
        Command::Down => down(),
        Command::Init => init(),
        Command::Help => cmd::help::help(),
        Command::Create(name) => create(name),
        Command::Status => status(),
        Command::Unknown(cmd) => println!("Unknown command or incorrect usage: {}", cmd),
    }
}
