use std::path::Path;
use std::process;
use std::{fs, io};

use crate::types::SessionMessage;

pub fn load_system_prompt() -> String {
    let file_contents = std::fs::read_to_string("system_prompt.txt");
    match file_contents {
        Ok(file_contents) => return file_contents,
        Err(e) => {
            eprintln!("failed to load file: {}", e);
            process::exit(1);
        }
    }
}

pub fn save_chat(session_state: &mut Vec<SessionMessage>, user_input: &str) {
    let mut args = user_input.split(' ');
    let path_string = args.nth(1).expect("/save expects a file path as argument");

    let path = Path::new(path_string);

    if path.exists() {
        println!(
            "----------------\nSystem: There's an existing file in this location. Are you sure that you want to override this file? (y/N)"
        );

        let mut res = String::new();
        io::stdin()
            .read_line(&mut res)
            .expect("Failed to read line");
        res = res.trim().to_string();

        match res.as_str() {
            res if res.ne("y") && res.ne("Y") => {
                println!("\n...Cancelling operation\n");
                return;
            }
            _ => {}
        }
    }

    println!("Attempting to save this session history to {}", path_string);
    let session_state_json = serde_json::to_string(session_state);
    match session_state_json {
        Ok(session_state_json) => {
            let res = fs::write(path_string, session_state_json);
            match res {
                Ok(_res) => {
                    println!("Session successfully saved to {}", path_string);
                }
                Err(e) => {
                    println!("Error while writing to path: {}, {}", path_string, e);
                }
            }
        }
        Err(e) => {
            println!("Error converting session state to JSON: {}", e);
        }
    }
}

pub fn load_chat(session_state: &mut Vec<SessionMessage>, user_input: &str) {}
