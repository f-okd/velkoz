use display_error_chain::DisplayErrorChain;
use std::io;
use std::process::ExitCode;
mod clients;
mod fs;
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
    let system_prompt = fs::load_system_prompt();
    let client = clients::gemini::Client::new(&system_prompt);

    println!("Hi, Vel'Koz here, what would you like to talk about today?\n");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input == "/quit" {
            break;
        }

        send_loading_indicator();

        let res = client.send_message_and_return_response(&user_input).await;
        println!("{}\n", res);
    }
    Ok(())
}

fn send_loading_indicator() {
    println!("Thinking...\n")
}
