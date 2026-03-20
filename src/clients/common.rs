pub trait ClientTrait {
    async fn send_message_and_return_response(&self, message: &str) -> String;
}
