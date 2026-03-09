use std::process;

pub fn load_system_prompt() -> String {
    let file_contents = std::fs::read_to_string("system_prompt.txt");
    match file_contents {
        Ok(file_contents)   => return file_contents,
        Err(e) => {
            eprintln!("failed to load file: {}", e);
            process::exit(1);
        }
    }
}