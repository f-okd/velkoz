pub enum Author {
    USER,
    MODEL,
}

pub struct SessionMessage {
    pub author: Author,
    pub message: String,
}
