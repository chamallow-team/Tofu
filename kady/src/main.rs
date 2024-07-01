mod config;
mod errors;

use std::path::{Path, PathBuf};
use clap::Parser;
use std::io;
use std::ops::DerefMut;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tokio::time;
use tracing::{error, info};
use tracing_subscriber::fmt::time::ChronoLocal;
use twilight_cache_inmemory::{InMemoryCache, InMemoryCacheBuilder, ResourceType};
use twilight_gateway::{ConfigBuilder as GatewayConfigBuilder, Shard};
use twilight_http::Client;
use twilight_model::channel::message::{AllowedMentions, MentionType};

#[derive(Parser, Clone, Debug)]
pub struct App {
    #[arg(short, long)]
    pub assets: PathBuf
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
async fn init_bot(assets_path: &Path){
    // We can read the token safely, as his presence has been checked before
    let token = std::fs::read_to_string(assets_path.join("token"))
        .expect("Cannot found file `token` in the assets folder");

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
                parse: vec![MentionType::Everyone, MentionType::Roles, MentionType::Users],
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
        |_, builder: GatewayConfigBuilder| builder.build()
    ).await;

    if let Err(e) = shards {
        error!(target: "AppInit", "Cannot create the shards: {e}");
        std::process::exit(1);
    }
    let shards = shards.unwrap().collect::<Vec<_>>();


    // Create the set of shards
    // Each shard will have his dedicated task
    let mut set = JoinSet::new();
    for mut shard in shards {
        let ctx = Context { client: client.clone(), cache: cache.clone() };
        set.spawn(async move {
            while let Ok(event) = shard.next_event().await {
                dbg!(&event.kind());

                // lastly, update the cache.
                // The cache needs to be updated at last
                // because some tasks may still have ownership over values
                // that will be updated.
                // This problem may cause some waiting, which is not wanted
                ctx.cache.write().await.update(&event);
            }
            println!("End of shard {}", shard.id())
        });
    }
    info!(target: "ShardBuilder", "{} shard(s) has been spawned", set.len());



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
    cache: Arc<RwLock<InMemoryCache>>
}