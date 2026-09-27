//! Oxide Mesh Net CLI Tool
//!
//! Provides operational management and diagnostic controls for oxide-mesh-net:
//! - `oxide status` : Inspects daemon health, PMTU, circuit breaker state, and port hopping.
//! - `oxide up`     : Brings interface online.
//! - `oxide down`   : Tears down interface.
//! - `oxide routes` : Displays active lock-free RCU Radix routing table.
//! - `oxide peers`  : Displays mesh peer connection statistics.
//! - `oxide acl`    : Dynamically inspects or reloads SIMD ACL policies.
//! - `oxide ping`   : Verifies daemon IPC latency and responsiveness.

use clap::{Args as ClapArgs, Parser, Subcommand};
use oxide_daemon::ipc::IpcClient;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(
    name = "oxide",
    about = "Management and diagnostic CLI for oxide-mesh-net",
    version
)]
struct Cli {
    /// Override path to daemon IPC Unix socket
    #[arg(short, long, global = true)]
    socket: Option<PathBuf>,

    /// Output results as raw JSON
    #[arg(long, global = true, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Query daemon status, PMTU, and transport state
    Status,

    /// Bring mesh network interface up
    Up {
        /// Optional configuration file path
        #[arg(short, long)]
        config: Option<String>,
    },

    /// Bring mesh network interface down
    Down,

    /// List active prefix routes in the lock-free Radix routing table
    Routes,

    /// Display connected peer telemetry and endpoints
    Peers,

    /// Manage access control list (ACL) rules
    Acl(AclCommand),

    /// Ping the daemon to test IPC connectivity
    Ping,
}

#[derive(ClapArgs, Debug)]
struct AclCommand {
    #[command(subcommand)]
    action: AclSubcommands,
}

#[derive(Subcommand, Debug)]
enum AclSubcommands {
    /// Reload ACL rules dynamically from a JSON file
    Reload {
        /// Path to ACL rules JSON file
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut client = match IpcClient::connect(cli.socket.as_deref()).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error connecting to oxide-daemon socket: {}", e);
            eprintln!("Ensure oxide-daemon is running (e.g., `oxide-daemon` or systemd service).");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Status => {
            let status = client.status().await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&status)?);
            } else {
                println!("=== Oxide Mesh Net Status ===");
                println!("  Node ID:                {}", status.node_id);
                println!("  TUN Interface:          {}", status.tun_name);
                println!("  Configured MTU:         {} bytes", status.mtu);
                println!("  Confirmed DPLPMTUD PMTU:{} bytes", status.confirmed_pmtu);
                println!("  Circuit Breaker State:  {}", status.circuit_breaker_state);
                println!("  Port Hopping Epoch:     #{}", status.port_hopping_epoch);
                println!("  Active Transport Port:  UDP/{}", status.current_port);
                println!(
                    "  DNS Self-Healing:       {}",
                    if status.dns_active {
                        "Active (.oxide)"
                    } else {
                        "Disabled"
                    }
                );
                println!(
                    "  TCP MSS Clamping:       {}",
                    if status.mss_clamping_active {
                        "Enabled (RFC 1624)"
                    } else {
                        "Disabled"
                    }
                );
                println!("  Daemon Uptime:          {}s", status.uptime_secs);
            }
        }
        Commands::Up { config } => {
            let res = client.up(config).await?;
            println!("{}", res);
        }
        Commands::Down => {
            let res = client.down().await?;
            println!("{}", res);
        }
        Commands::Routes => {
            let routes = client.routes().await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&routes)?);
            } else {
                println!("=== Active Radix Prefix Routing Table ===");
                if routes.is_empty() {
                    println!("  (No active prefix routes configured)");
                } else {
                    println!(
                        "{:<24} {:<38} {:<8} {:<8}",
                        "PREFIX", "TARGET NODE", "PMTU", "DIRECT"
                    );
                    println!("{:-<24} {:-<38} {:-<8} {:-<8}", "", "", "", "");
                    for r in routes {
                        println!(
                            "{:<24} {:<38} {:<8} {:<8}",
                            r.prefix, r.target_node, r.pmtu, r.is_direct
                        );
                    }
                }
            }
        }
        Commands::Peers => {
            let peers = client.peers().await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&peers)?);
            } else {
                println!("=== Connected Mesh Peers ===");
                if peers.is_empty() {
                    println!("  (No peers currently connected)");
                } else {
                    println!(
                        "{:<38} {:<18} {:<24} {:<8}",
                        "NODE ID", "OVERLAY IP", "ENDPOINT", "PMTU"
                    );
                    println!("{:-<38} {:-<18} {:-<24} {:-<8}", "", "", "", "");
                    for p in peers {
                        let ep = p.endpoint.unwrap_or_else(|| "none".into());
                        println!(
                            "{:<38} {:<18} {:<24} {:<8}",
                            p.node_id, p.overlay_ip, ep, p.pmtu
                        );
                    }
                }
            }
        }
        Commands::Acl(AclCommand { action }) => match action {
            AclSubcommands::Reload { file } => {
                let rules_json = if let Some(path) = file {
                    Some(std::fs::read_to_string(path)?)
                } else {
                    None
                };
                let res = client.acl_reload(rules_json).await?;
                println!("{}", res);
            }
        },
        Commands::Ping => {
            let start = Instant::now();
            let pong = client.ping().await?;
            let rtt = start.elapsed();
            if pong {
                println!("Pong from oxide-daemon (IPC RTT: {:.3?})", rtt);
            } else {
                eprintln!("Daemon returned unexpected ping response");
            }
        }
    }

    Ok(())
}
