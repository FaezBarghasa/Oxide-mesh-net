//! Views root module for oxide-ui

pub mod audit;
pub mod dashboard;
pub mod identity;
pub mod obfuscation;
pub mod platform;
pub mod routing;
pub mod services;

pub use audit::AuditView;
pub use dashboard::DashboardView;
pub use identity::IdentityView;
pub use obfuscation::ObfuscationView;
pub use platform::PlatformView;
pub use routing::RoutingView;
pub use services::ServicesView;
