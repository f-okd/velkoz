use super::common::ClientTrait;
use gemini_rust::Gemini;
use std::env;

pub struct Client {
    client: Gemini,
    system_prompt: String,
}

impl Client {
    pub fn new(system_prompt: &str) -> Self {
        let api_key =
            env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
        let client_res = Gemini::new(api_key);

        Self {
            client: match client_res {
                Ok(client) => client,
                Err(error) => panic!("Problem initialising Gemini Client: {error:?}"),
            },
            system_prompt: system_prompt.to_string(),
        }
    }
}
impl ClientTrait for Client {
    async fn send_message_and_return_response(&self, message: &str) -> String {
        let res = self
            .client
            .generate_content()
            .with_system_prompt(&self.system_prompt)
            .with_user_message(message)
            .execute()
            .await;

        return match res {
            Ok(res) => res.text(),
            Err(error) => panic!("Problem fetching response from Gemini: {error:?}"),
        };
    }
}
