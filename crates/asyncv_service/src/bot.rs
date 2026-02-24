use async_trait::async_trait;

use anyhow::anyhow;

#[derive(Clone, Debug)]
pub struct IncomingMessage {
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct OutgoingMessage {
    pub content: String,
}

#[async_trait]
pub trait Bot: Send + Sync {
    type Error: Send + Sync + 'static;

    async fn listen(&self) -> Result<IncomingMessage, Self::Error>;
    async fn send(&self, message: OutgoingMessage) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug)]
pub struct NullBot;

#[async_trait]
impl Bot for NullBot {
    type Error = anyhow::Error;

    async fn listen(&self) -> Result<IncomingMessage, Self::Error> {
        Err(anyhow!("bot is not configured"))
    }

    async fn send(&self, _message: OutgoingMessage) -> Result<(), Self::Error> {
        Err(anyhow!("bot is not configured"))
    }
}
