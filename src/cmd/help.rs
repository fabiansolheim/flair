const FLAIR: &str = r#"
    ______      _     
   / __/ /___ _(_)____
  / /_/ / __ `/ / ___/
 / __/ / /_/ / / /    
/_/ /_/\__,_/_/_/     
                     
"#;

pub fn help() {
    println!("\x1b[34m{} \x1b[0m", FLAIR);
    println!("Usage:");
    println!("    \x1b[34mflair init \x1b[0m- Initialize a new migration directory");
    println!("    \x1b[34mflair create <name> \x1b[0m- Create a new migration file");
    println!("    \x1b[34mflair up \x1b[0m- Run all pending migrations");
    println!("    \x1b[34mflair down \x1b[0m- Reverts last applied migration");
    println!("    \x1b[34mflair status \x1b[0m- Show the status of all migrations");
    println!("    \x1b[34mflair help \x1b[0m- Prints this message");
}
