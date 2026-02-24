use sea_orm::DerivePartialModel;

use crate::bot::{Bot, IncomingMessage, OutgoingMessage};
use crate::entity;

pub struct Agent<B: Bot> {
    pub config: entity::agent::ModelEx,
    pub bot: B,
}

impl<B: Bot> Agent<B> {
    pub fn new(config: entity::agent::ModelEx, bot: B) -> Self {
        Self { config, bot }
    }

    pub fn bot(&self) -> &B {
        &self.bot
    }

    pub fn config(&self) -> &entity::agent::ModelEx {
        &self.config
    }

    pub async fn listen(&self) -> Result<IncomingMessage, <B as Bot>::Error> {
        self.bot.listen().await
    }

    pub async fn send(&self, message: OutgoingMessage) -> Result<(), <B as Bot>::Error> {
        self.bot.send(message).await
    }
}
