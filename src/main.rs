use display_error_chain::DisplayErrorChain;
use gemini_rust::Gemini;
use std::env;
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

    let res = client
        .generate_content()
        .with_system_prompt(system_prompt)
        .with_user_message("What makes Rust a good choice for systems programming?")
        .execute()
        .await?;

    println!("{}", res.text());

    Ok(())
}