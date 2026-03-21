use core::panic;
use display_error_chain::DisplayErrorChain;
use std::env;
use std::io;
use std::process;
use std::process::ExitCode;
mod clients;
mod types;
mod velkoz_fs;
use clients::common::ClientTrait;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    match do_main().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let error_chain = DisplayErrorChain::new(e.as_ref());
            tracing::error!(error.debug = ?e, error.chained = %error_chain, "execution failed");
            ExitCode::FAILURE
        }
    }
}

async fn do_main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client;
    let system_prompt = velkoz_fs::load_system_prompt();
    let provider = parse_cli_args();
    let mut session_state: Vec<types::SessionMessage> = Vec::new();

    match provider.as_str() {
        // Feel free to add support for other LLMs here. I personally can't be arsed.
        "--gemini" => client = clients::gemini::Client::new(&system_prompt),
        _ => {
            println!("No LLM specified, or unrecognised option. Defaulting to Gemini.");
            client = clients::gemini::Client::new(&system_prompt)
        }
    }

    println!("Hi, Vel'Koz here, what would you like to talk about today?\n");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input = user_input.trim().to_string();

        match user_input.as_str() {
            user_input if user_input.starts_with("/quit") => break,
            user_input if user_input.starts_with("/save") => {
                velkoz_fs::save_chat(&mut session_state, user_input);
            }
            user_input if user_input.starts_with("/load") => {
                velkoz_fs::load_chat(&mut session_state, user_input);
            }
            _ => {
                send_loading_indicator();

                let res = client
                    .send_message_and_return_response(&mut session_state, &user_input)
                    .await;
                println!("----------------\nVel'Koz: {}\n", res);
            }
        }
    }
    Ok(())
}

fn parse_cli_args() -> String {
    let mut cli_args = env::args();
    let arg_len = cli_args.len();

    match arg_len {
        // User passed no arguments
        1 => {
            return "Gemini".to_string();
        }
        // User passed more than 1 argument
        x if x > 2 => {
            panic!(
                "Vel'Koz only supports 1 CLI argument. Either --help or your chosen LLM provider; currently only --gemini is supported"
            );
        }
        _ => {
            let arg = cli_args.nth(1).unwrap().to_lowercase();
            if arg == "--help" {
                println!(
                    r#"
                Vel'Koz - Experimental CLI tool for non-directive therapy

                USAGE:
                    velkoz [OPTIONS]

                OPTIONS:
                    --gemini    Use Google Gemini as the LLM provider (default)
                    --help      Print this help message

                COMMANDS (during chat):
                    /quit       Exit the program

                SETUP:
                    Requires a GEMINI_API_KEY environment variable, or a .env file containing it.
                    Customize Vel'Koz's personality by editing system_prompt.txt.
                "#
                );
                process::exit(0);
            } else {
                return arg;
            }
        }
    }
}

fn send_loading_indicator() {
    println!("\nThinking...\n")
}
