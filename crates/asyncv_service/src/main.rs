use sea_orm::{Database, DatabaseConnection, DerivePartialModel, EntityTrait, RelationTrait};
use std::future;

pub mod agent;
mod bot;
mod entity;



async fn init_agents<B, F>(
    db: &DatabaseConnection,
    bot_factory: F,
) -> anyhow::Result<Vec<agent::Agent<B>>>
where
    B: bot::Bot,
    F: Fn(&entity::agent::Model) -> B,
{
    let agents = entity::agent::Entity::find()
        .left_join(entity::language_model_config::Entity)
        .into_partial_model()
        .all(db)
        .await?;

    let initialized: Vec<agent::Agent<B>> = agents
        .into_iter()
        .map(|agent| {
            let bot = bot_factory(&agent);
            agent::Agent::new(agent.into(), bot)
        })
        .collect();

    Ok(initialized)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let env_path = manifest_dir.join(".env");
    dotenvy::from_path(&env_path).ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db = Database::connect(&db_url).await?;

    let initialized_agents = init_agents(&db, |_| bot::NullBot).await?;
    let models_initialized = initialized_agents
        .iter()
        .filter(|agent| agent.config.language_model_config_id.is_some())
        .count();
    println!(
        "initialized agents: {} (language models: {})",
        initialized_agents.len(),
        models_initialized
    );

    future::pending::<()>().await;
    Ok(())
}
