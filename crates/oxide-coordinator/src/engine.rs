//! Coordinator engine with embedded MQTT broker and Actix Web server

use std::sync::Arc;
use actix_web::{web, App, HttpServer, middleware};
use actix_web::dev::ServerHandle;
use actix_web_prometheus::PrometheusMetricsBuilder;
use rumqttd::Broker;
use tokio::sync::mpsc;
use tracing::{info, error};
use crate::{
    config::CoordinatorConfig,
    error::{CoordinatorError, Result},
    auth::AuthService,
    storage::Storage,
    handlers,
};
use oxide_core::MeshName;

/// Main coordinator instance
pub struct Coordinator {
    config: CoordinatorConfig,
    http_handle: Option<ServerHandle>,
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
            http_handle: None,
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
        let config = rumqttd::Config::default();
        let mut broker = Broker::new(config);

        // Spawn broker task
        tokio::spawn(async move {
            if let Err(e) = broker.start() {
                error!("MQTT broker error: {:?}", e);
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
                .app_data(web::Data::new(mesh_name.clone()))
                .service(
                    web::scope("/api/v1")
                        .configure(handlers::configure_routes)
                )
                .service(
                    web::scope("/health")
                        .route("", web::get().to(handlers::health::health_check))
                        .route("/ready", web::get().to(handlers::health::readiness_check))
                        .route("/live", web::get().to(handlers::health::liveness_check))
                )
                .service(
                    web::scope("/metrics")
                        .route("", web::get().to(handlers::metrics::metrics_endpoint))
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

        let handle = server.handle();
        tokio::spawn(server);

        self.http_handle = Some(handle);
        info!("HTTP server started on {}", http_config.bind);
        Ok(())
    }

    /// Stop the coordinator
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping coordinator...");

        if let Some(handle) = self.http_handle.take() {
            handle.stop(true).await;
        }

        info!("Coordinator stopped");
        Ok(())
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
    pub mesh_name: MeshName,
}