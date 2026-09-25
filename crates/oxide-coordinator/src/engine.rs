//! Coordinator engine with embedded MQTT broker and Actix Web server

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use actix_web::{web, App, HttpServer, middleware, HttpResponse, Responder};
use actix_web::dev::Server;
use actix_web_prom::PrometheusMetricsBuilder;
use rumqttd::{Broker, BrokerHandle, Config as MqttConfig};
use rumqttd::config::ConfigBuilder;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, error};
use crate::{
    config::{CoordinatorConfig, MqttListenerConfig, HttpConfig, AuthConfig, StorageConfig},
    error::{CoordinatorError, Result},
    auth::{AuthService, Claims},
    storage::Storage,
    handlers::{health, metrics, enrollment, nodes, acl, routes},
};
use oxide_core::{NodeId, MeshName};
use oxide_protocol::topics::*;

/// Main coordinator instance
pub struct Coordinator {
    config: CoordinatorConfig,
    broker_handle: Option<BrokerHandle>,
    http_server: Option<Server>,
    auth_service: Arc<AuthService>,
    storage: Arc<Storage>,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl Coordinator {
    /// Create a new coordinator
    pub async fn new(config: CoordinatorConfig) -> Result<Self> {
        let auth_service = Arc::new(AuthService::new(&config.auth)?);
        let storage = Arc::new(Storage::new(&config.storage)?);

        Ok(Self {
            config,
            broker_handle: None,
            http_server: None,
            auth_service,
            storage,
            shutdown_tx: None,
        })
    }

    /// Start the coordinator
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting oxide-coordinator for mesh: {}", self.config.mesh_name);

        // Start MQTT broker
        self.start_mqtt_broker().await?;

        // Start Actix Web HTTP server
        self.start_http_server().await?;

        info!("Coordinator started successfully");
        Ok(())
    }

    /// Start MQTT broker
    async fn start_mqtt_broker(&mut self) -> Result<()> {
        let mut builder = ConfigBuilder::default();

        for listener in &self.config.mqtt_listeners {
            let mut listener_config = rumqttd::config::ListenerConfig::default();
            listener_config.bind = listener.bind;
            listener_config.max_connections = listener.max_connections;
            listener_config.max_packet_size = listener.max_packet_size;
            
            if listener.tls {
                // TLS config would go here
            }

            builder = builder.add_listener(listener_config);
        }

        // Set up auth hook
        let auth_service = self.auth_service.clone();
        builder = builder.auth(move |connection| {
            let auth_service = auth_service.clone();
            Box::pin(async move {
                auth_service.authenticate_mqtt(connection).await
            })
        });

        let config = builder.build().map_err(|e| CoordinatorError::Mqtt(e))?;
        let (broker, handle) = Broker::new(config);
        self.broker_handle = Some(handle);

        // Spawn broker task
        tokio::spawn(async move {
            if let Err(e) = broker.start().await {
                error!("MQTT broker error: {}", e);
            }
        });

        info!("MQTT broker started on {} listeners", self.config.mqtt_listeners.len());
        Ok(())
    }

    /// Start Actix Web HTTP server
    async fn start_http_server(&mut self) -> Result<()> {
        let http_config = self.config.http.clone();
        let auth_service = self.auth_service.clone();
        let storage = self.storage.clone();
        let mesh_name = self.config.mesh_name.clone();
        let broker_handle = self.broker_handle.clone().unwrap();

        let prometheus = PrometheusMetricsBuilder::new("oxide_coordinator")
            .endpoint("/metrics")
            .build()
            .map_err(|e| CoordinatorError::Internal(e.to_string()))?;

        let server = HttpServer::new(move || {
            let cors = if http_config.cors {
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
            } else {
                actix_cors::Cors::default()
            };

            App::new()
                .wrap(middleware::Logger::default())
                .wrap(middleware::Compress::default())
                .wrap(prometheus.clone())
                .wrap(cors)
                .app_data(web::Data::new(auth_service.clone()))
                .app_data(web::Data::new(storage.clone()))
                .app_data(web::Data::new(broker_handle.clone()))
                .app_data(web::Data::new(mesh_name.clone()))
                .service(
                    web::scope("/api/v1")
                        .configure(handlers::configure_routes)
                )
                .service(
                    web::scope("/health")
                        .route("", web::get().to(health::health_check))
                        .route("/ready", web::get().to(health::readiness_check))
                        .route("/live", web::get().to(health::liveness_check))
                )
                .service(
                    web::scope("/metrics")
                        .route("", web::get().to(metrics::metrics_endpoint))
                )
                // Serve static files (Dioxus WASM)
                .service(
                    actix_files::Files::new("/", http_config.static_dir.clone().unwrap_or_else(|| "./static".into()))
                        .index_file("index.html")
                        .prefer_utf8(true)
                )
        })
        .bind(http_config.bind)?
        .workers(http_config.workers)
        .run();

        self.http_server = Some(server);
        info!("HTTP server started on {}", http_config.bind);
        Ok(())
    }

    /// Stop the coordinator
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping coordinator...");

        if let Some(server) = self.http_server.take() {
            server.stop(true).await;
        }

        if let Some(handle) = self.broker_handle.take() {
            // Broker doesn't have explicit stop, just drop
        }

        info!("Coordinator stopped");
        Ok(())
    }

    /// Get broker handle
    pub fn broker_handle(&self) -> Option<&BrokerHandle> {
        self.broker_handle.as_ref()
    }

    /// Wait for shutdown signal
    pub async fn wait_for_shutdown(&mut self) {
        let (tx, mut rx) = mpsc::channel(1);
        self.shutdown_tx = Some(tx);
        rx.recv().await;
    }
}

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<AuthService>,
    pub storage: Arc<Storage>,
    pub broker: BrokerHandle,
    pub mesh_name: MeshName,
}