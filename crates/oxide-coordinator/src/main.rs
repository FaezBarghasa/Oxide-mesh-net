//! oxide-coordinator standalone daemon binary
//!
//! Embedded MQTT broker, SurrealDB 3 multi-model persistence, and Actix Web REST/WS coordinator.

use clap::Parser;
use oxide_coordinator::{Coordinator, CoordinatorConfig, HttpConfig, StorageBackendType, StorageConfig};
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "oxide-coordinator")]
#[command(author = "Faez Barghasa <faez.barghasa@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Embedded MQTT broker and Actix Web coordinator for oxide-mesh-net", long_about = None)]
struct Cli {
    /// Mesh network name
    #[arg(short, long, default_value = "oxide")]
    mesh: String,

    /// HTTP / Actix Web API bind address
    #[arg(short, long, default_value = "0.0.0.0:8080")]
    bind: SocketAddr,

    /// Storage backend type (mem, surrealkv, ws)
    #[arg(long, default_value = "mem")]
    storage: String,

    /// SurrealDB database namespace
    #[arg(long, default_value = "oxide")]
    namespace: String,

    /// SurrealDB database name
    #[arg(long, default_value = "coordinator")]
    database: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    info!("Initializing oxide-coordinator for mesh '{}'...", cli.mesh);

    let storage_backend = match cli.storage.to_lowercase().as_str() {
        "mem" => StorageBackendType::Memory,
        "surrealkv" => StorageBackendType::SurrealKv,
        "ws" => StorageBackendType::SurrealWs,
        _ => StorageBackendType::Memory,
    };

    let mut config = CoordinatorConfig::default();
    config.mesh_name = cli.mesh;
    config.http = HttpConfig {
        bind: cli.bind,
        workers: 4,
        cors: true,
        static_dir: None,
    };
    config.storage = StorageConfig {
        backend: storage_backend,
        namespace: cli.namespace,
        database: cli.database,
        path: None,
        endpoint: None,
        username: None,
        password: None,
    };

    let mut coordinator = Coordinator::new(config).await?;
    coordinator.start().await?;

    info!("Oxide coordinator running on http://{}. Press Ctrl+C to terminate.", cli.bind);
    tokio::signal::ctrl_c().await?;

    info!("Shutting down coordinator...");
    coordinator.stop().await?;
    info!("Coordinator shutdown complete.");

    Ok(())
}
