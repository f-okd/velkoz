use async_trait::async_trait;

use crate::types::SessionMessage;

#[async_trait]
pub trait ClientTrait {
    async fn send_message_and_return_response(
        &self,
        session_state: &mut Vec<SessionMessage>,
        message: &str,
    ) -> String;
}
