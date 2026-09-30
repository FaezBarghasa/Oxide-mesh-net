//! oxide-coordinator standalone daemon binary
//!
//! Embedded MQTT broker, SurrealDB 3 multi-model persistence, and Actix Web REST/WS coordinator.

use clap::Parser;
use oxide_coordinator::{
    Coordinator, CoordinatorConfig, HttpConfig, StorageBackendType, StorageConfig,
};
use std::net::SocketAddr;
use tracing::info;
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
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    info!("Initializing oxide-coordinator for mesh '{}'...", cli.mesh);

    let storage_backend = match cli.storage.to_lowercase().as_str() {
        "mem" => StorageBackendType::SurrealMem,
        "surrealkv" => StorageBackendType::SurrealKv,
        "ws" => StorageBackendType::SurrealWs,
        _ => StorageBackendType::SurrealMem,
    };

    let config = CoordinatorConfig {
        mesh_name: cli.mesh,
        http: HttpConfig {
            bind: cli.bind,
            workers: 4,
            tls: false,
            cert_path: None,
            key_path: None,
            request_timeout: std::time::Duration::from_secs(30),
            body_limit: 1024 * 1024,
            cors: true,
            static_dir: None,
        },
        storage: StorageConfig {
            backend: storage_backend,
            data_dir: std::path::PathBuf::from("/var/lib/oxide-coordinator"),
            surreal_url: None,
            surreal_ns: cli.namespace,
            surreal_db: cli.database,
            surreal_user: None,
            surreal_pass: None,
            raft: None,
        },
        ..Default::default()
    };

    let mut coordinator = Coordinator::new(config).await?;
    coordinator.start().await?;

    info!(
        "Oxide coordinator running on http://{}. Press Ctrl+C to terminate.",
        cli.bind
    );
    tokio::signal::ctrl_c().await?;

    info!("Shutting down coordinator...");
    coordinator.stop().await?;
    info!("Coordinator shutdown complete.");

    Ok(())
}
