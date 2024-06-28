// use clap::{App, Arg, SubCommand};
use dialoguer::{Input, Select};
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("\n\n\nHello, welcome to read_file!\n\n");

    let options = vec!["Yes", "No"];
    let selection = Select::new()
        .with_prompt("Is the file you want to read in the home directory?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Failed to read selection");

    let dir_path = if selection == 0 {
        env::var("HOME").expect("Home environment variable is not set")
    } else {
        Input::new()
            .with_prompt("Please enter the full path name to a directory (or press Enter to use the home directory)")
            .allow_empty(true)
            .interact_text()
            .expect("Failed to read directory path")
    };

    let dir_path = if dir_path.is_empty() {
        env::var("HOME").expect("Home environment variable is not set")
    } else {
        dir_path
    };

    let file_name: String = Input::new()
        .with_prompt("Please enter the file name to read")
        .interact_text()
        .expect("Failed to read file name");

    let file_path = format!("{}/{}", dir_path, file_name);
    println!("{}", file_path);
    match read_file_to_string(&file_path) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
