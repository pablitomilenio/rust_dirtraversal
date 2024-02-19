use dirs;
use std::env;

fn locate() {
    match env::current_dir() {
        Ok(dir) => {
            println!("Current directory: {}", dir.display());
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    match dirs::home_dir() {
        Some(path) => println!("Your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
}

fn main() {
    locate();
}
