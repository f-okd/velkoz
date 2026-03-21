use crate::types::{Author, SessionMessage};

use super::common::ClientTrait;
use gemini_rust::{ContentBuilder, Gemini};
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
    async fn send_message_and_return_response(
        &self,
        session_state: &mut Vec<SessionMessage>,
        message: &str,
    ) -> String {
        session_state.push(SessionMessage {
            author: Author::USER,
            message: message.to_string(),
        });

        let mut chat_stub: ContentBuilder = self
            .client
            .generate_content()
            .with_system_prompt(&self.system_prompt);

        for message in session_state.iter() {
            chat_stub = match message.author {
                Author::USER => chat_stub.with_user_message(message.message.as_str()),
                Author::MODEL => chat_stub.with_model_message(message.message.as_str()),
            }
        }

        let res: Result<gemini_rust::GenerationResponse, gemini_rust::ClientError> =
            chat_stub.execute().await;

        return match res {
            Ok(res) => {
                session_state.push(SessionMessage {
                    author: Author::MODEL,
                    message: res.text(),
                });
                res.text()
            }
            Err(error) => panic!("Problem fetching response from Gemini: {error:?}"),
        };
    }
}
