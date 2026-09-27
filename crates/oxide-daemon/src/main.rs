//! Oxide Mesh Net Daemon Binary
//!
//! Production-grade background daemon coordinating multi-queue TUN interfaces,
//! dynamic MSS clamping, lock-free RCU Radix routing, DPLPMTUD probing, and secure IPC.

use clap::Parser;
use oxide_daemon::config::DaemonConfig;
use oxide_daemon::engine::DaemonEngine;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

#[derive(Parser, Debug)]
#[command(
    name = "oxide-daemon",
    about = "High-performance background daemon for oxide-mesh-net"
)]
struct Args {
    /// Path to JSON configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Override TUN interface name
    #[arg(long)]
    tun: Option<String>,

    /// Override interface MTU
    #[arg(long)]
    mtu: Option<u16>,

    /// Override UDP listen port
    #[arg(long)]
    port: Option<u16>,

    /// Custom Unix IPC socket path
    #[arg(long)]
    socket: Option<PathBuf>,

    /// Disable OS DNS watchdog
    #[arg(long, default_value_t = false)]
    no_dns: bool,

    /// Disable dynamic TCP MSS clamping
    #[arg(long, default_value_t = false)]
    no_mss_clamp: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    // Load base config or default
    let mut config = if let Some(ref config_path) = args.config {
        info!("Loading configuration from {:?}", config_path);
        DaemonConfig::load_from_file(config_path)?
    } else {
        info!("No configuration file specified; using defaults");
        DaemonConfig::default()
    };

    // Apply CLI overrides
    if let Some(tun) = args.tun {
        config.tun_name = tun;
    }
    if let Some(mtu) = args.mtu {
        config.mtu = mtu;
    }
    if let Some(port) = args.port {
        config.listen_port = port;
    }
    if let Some(socket) = args.socket {
        config.socket_path = Some(socket);
    }
    if args.no_dns {
        config.enable_dns = false;
    }
    if args.no_mss_clamp {
        config.enable_mss_clamp = false;
    }

    info!(
        "Initializing Oxide Daemon [Node: {}, TUN: {}, MTU: {}, Port: {}]",
        config.node_id, config.tun_name, config.mtu, config.listen_port
    );

    let engine = Arc::new(DaemonEngine::new(config));
    DaemonEngine::start(engine.clone()).await?;

    info!("Oxide Daemon is running. Waiting for shutdown signal...");

    // Await termination signal
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm = signal(SignalKind::terminate())?;
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                info!("Received SIGINT (Ctrl+C); shutting down daemon gracefully...");
            }
            _ = sigterm.recv() => {
                info!("Received SIGTERM; shutting down daemon gracefully...");
            }
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await?;
        info!("Received Ctrl+C; shutting down daemon gracefully...");
    }

    info!("Oxide Daemon terminated cleanly.");
    Ok(())
}
