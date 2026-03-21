use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::types::{Author, SessionMessage};

use super::common::ClientTrait;
use std::env;

#[derive(Serialize)]
enum OllamaMessageRolesType {
    SYSTEM,
    USER,
    ASSISTANT,
    // TOOL,
}

#[derive(Serialize)]
struct OllamaMessagesType {
    role: OllamaMessageRolesType,
    content: String,
}

#[derive(Serialize)]
struct OllamaHttpSendBody {
    model: String,
    messages: Vec<OllamaMessagesType>,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaMessageType {
    content: String,
}

#[derive(Deserialize)]
struct OllamaHttpRecvBody {
    message: OllamaMessageType,
}
pub struct Client {
    model: String,
    system_prompt: String,
}

impl Client {
    pub fn new(system_prompt: &str) -> Self {
        let model = env::var("OLLAMA_MODEL").expect("OLLAMA_MODEL environment variable not set");

        Self {
            model,
            system_prompt: system_prompt.to_string(),
        }
    }

    async fn send_request(
        &self,
        ollama_formatted_messages: Vec<OllamaMessagesType>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let body = OllamaHttpSendBody {
            model: self.model.clone(),
            messages: ollama_formatted_messages,
            stream: false,
        };
        let res: String = ureq::post("http://localhost:11434/api/chat")
            .send_json(body)?
            .body_mut()
            .read_to_string()?;

        return Ok(res);
    }
}

#[async_trait]
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

        let mut ollama_formatted_state: Vec<OllamaMessagesType> = session_state
            .iter()
            .map(|s| OllamaMessagesType {
                role: match s.author {
                    Author::MODEL => OllamaMessageRolesType::ASSISTANT,
                    Author::USER => OllamaMessageRolesType::USER,
                },
                content: s.message.clone(),
            })
            .collect();

        ollama_formatted_state.insert(
            0,
            OllamaMessagesType {
                role: OllamaMessageRolesType::SYSTEM,
                content: self.system_prompt.clone(),
            },
        );

        let res_1 = &self.send_request(ollama_formatted_state).await;
        match res_1 {
            Ok(res_1) => {
                let res_2 = serde_json::from_str(res_1);
                let body: OllamaHttpRecvBody;
                match res_2 {
                    Ok(res_2) => body = res_2,
                    Err(e) => panic!("Failed to parse response from ollama client: {}", e),
                };
                return body.message.content;
            }
            Err(e) => panic!("Failed to get response from ollama client: {}", e),
        }
    }
}
