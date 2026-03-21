use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Author {
    USER,
    MODEL,
}

#[derive(Serialize, Deserialize)]
pub struct SessionMessage {
    pub author: Author,
    pub message: String,
}
