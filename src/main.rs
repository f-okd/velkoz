use display_error_chain::DisplayErrorChain;
use gemini_rust::Gemini;
use std::env;
use std::io;
use std::process::ExitCode;
mod fs;

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
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let client = Gemini::new(api_key)?;
    let system_prompt = fs::load_system_prompt();

    println!("Hi, Velk'oz here, what would you like to talk about today?\n");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input == "/quit" {
            break;
        }

        send_loading_indicator();

        let res = send_message_and_get_response(&user_input, &system_prompt, &client).await?;
        println!("{}\n", res);
    }
    Ok(())
}

async fn send_message_and_get_response(
    message: &str,
    system_prompt: &str,
    client: &Gemini,
) -> Result<String, Box<dyn std::error::Error>> {
    let res = client
        .generate_content()
        .with_system_prompt(system_prompt)
        .with_user_message(message)
        .execute()
        .await?;
    Ok(res.text())
}

fn send_loading_indicator() {
    println!("Thinking...\n")
}
