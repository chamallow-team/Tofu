mod config;
mod errors;
mod controllers;
mod plugins;

use std::path::{Path, PathBuf};
use clap::Parser;
use std::{env, io};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tokio::time;
use tracing::{error, info};
use tracing_subscriber::fmt::time::ChronoLocal;
use twilight_cache_inmemory::{InMemoryCache, InMemoryCacheBuilder, ResourceType};
use twilight_gateway::{ConfigBuilder as GatewayConfigBuilder};
use twilight_http::Client;
use twilight_model::channel::message::{AllowedMentions, MentionType};
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;
use crate::controllers::database;
use crate::controllers::database::Database;

#[derive(Parser, Clone, Debug)]
pub struct App {
    #[arg(short, long)]
    pub assets: PathBuf,
}

/// This function will check if the arguments passed to the program are valid
fn check_app_args(app: &App) -> io::Result<()> {
    if !app.assets.exists() || !std::fs::metadata(&app.assets)?.is_dir() {
        error!(target: "AssetsPreCheck", "The given assets folder path is invalid");
        std::process::exit(1);
    }

    let config_path = app.assets.join("config.toml");
    if !config_path.exists() || !std::fs::metadata(&config_path)?.is_file() {
        error!(target: "AssetsPreCheck", "Cannot find the required `config.toml` file in the assets folder");
        std::process::exit(1);
    }

    let token_path = app.assets.join("token");
    if !token_path.exists() || !std::fs::metadata(&token_path)?.is_file() {
        error!(target: "AssetsPreCheck", "Cannot find the required `token` file in the 'assets' folder");
        std::process::exit(1);
    }


    info!(
        target: "App",
        "Config located at {}",
        config_path.canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    );
    info!(
        target: "App",
        "Token  located at {}",
        token_path.canonicalize()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    // A MUST; will trace back every panic that occurs
    // See https://crates.io/crates/color-eyre
    color_eyre::install()?;

    dotenvy::dotenv()?;

    // init tracing_subscriber
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_timer(ChronoLocal::default())
        .init();

    let app = App::parse();

    if let Err(e) = check_app_args(&app) {
        error!(target: "App", "Cannot init the app: {e}");
        std::process::exit(1);
    };

    init_bot(&app.assets).await;

    Ok(())
}

/// Start the bot and all related services
async fn init_bot(assets_path: &Path) {
    // read the token from .env
    let token = env::var("TOKEN").expect("Cannot acquire the 'TOKEN' env");
    let database_url = env::var("DATABASE_URL").expect("Cannot acquire the 'DATABASE_URL' env");

    let database = match database::Database::connect(&database_url).await {
        Ok(pool) => Arc::new(pool),
        Err(e) => {
            error!(target: "Database", "Error connecting to database: {e}");
            std::process::exit(1);
        }
    };

    let config = match config::get_config(assets_path) {
        Ok(cnf) => cnf,
        Err(e) => {
            error!(target: "InitApp", "Cannot load the config file: {e:?}");
            std::process::exit(1);
        }
    };

    // Create an in-memory cache instance
    let cache = Arc::new(RwLock::new(
        InMemoryCacheBuilder::new()
            .message_cache_size(config.cache.message_cache_size)
            .resource_types(ResourceType::all())
            .build()
    ));

    let client = Arc::new(
        Client::builder()
            .token(token.clone())
            .timeout(Duration::from_secs(5))
            .default_allowed_mentions(AllowedMentions {
                parse: vec![MentionType::Roles, MentionType::Users],
                replied_user: false,
                roles: vec![],
                users: vec![],
            })
            .build()
    );

    // Create the shards
    let gateway_config = GatewayConfigBuilder::new(token, config.intents).build();
    let shards = twilight_gateway::stream::create_recommended(
        &client,
        gateway_config,
        // If we need to change config for each shard
        |_, builder: GatewayConfigBuilder| builder.build(),
    ).await;

    let application_id = {
        let response = client.current_user_application().await.expect("Couldn't get the current application");
        response.model().await.expect("Cannot acquire the Application Model").id
    };

    if let Err(e) = shards {
        error!(target: "AppInit", "Cannot create the shards: {e}");
        std::process::exit(1);
    }
    let shards = shards.unwrap().collect::<Vec<_>>();

    // Create the set of shards
    // Each shard will have his dedicated task
    let mut set = JoinSet::new();
    for mut shard in shards {
        let ctx_s = Context { client: client.clone(), cache: cache.clone(), database: database.clone(), application_id };
        set.spawn(async move {
            loop {
                // acquire the event and manage any error received
                let event = match shard.next_event().await {
                    Ok(event) => event,
                    Err(source) => {
                        error!(?source, "error receiving event");
                        if source.is_fatal() { break; }

                        continue;
                    }
                };

                let ctx = ctx_s.clone();
                tokio::task::spawn(async move {
                    dbg!(&event.kind());
                    let r = controllers::event_receiver::handle_event(&ctx, &event).await;

                    if let Err(e) = r {
                        error!(target: "App", "Error handling event: {e:?}");
                    }

                    // lastly, update the cache.
                    // The cache needs to be updated at last
                    // because some tasks may still have ownership over values
                    // that will be updated.
                    // This problem may cause some waiting, which is not wanted
                    ctx.cache.write().await.update(&event);
                });
            }

            println!("End of shard {}", shard.id())
        });
    }
    info!(target: "ShardBuilder", "{} shard(s) has been spawned", set.len());

    // Wait for all shards to finish
    // A restart logic should be implemented
    while (set.join_next().await).is_some() {
        time::sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Clone)]
/// Contains useful resources that may be required for events and commands
pub struct Context {
    // We need to find a better solution to access to the shards
    // shards: Arc<RwLock<Vec<Shard>>>,
    client: Arc<Client>,
    cache: Arc<RwLock<InMemoryCache>>,
    database: Arc<Database>,
    application_id: Id<ApplicationMarker>,
}