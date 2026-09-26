use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::{AuthConfig, error::{CoordinatorError, Result}};
use oxide_core::{NodeId, MeshName};
use oxide_crypto::keys::DeviceIdentityPublicKey;

/// JWT claims for node authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // Node ID
    pub mesh: String,          // Mesh name
    pub iat: u64,              // Issued at
    pub exp: u64,              // Expiration
    pub capabilities: Vec<String>,
    pub scopes: Vec<String>,
}

/// Authentication service
pub struct AuthService {
    config: AuthConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    enrollment_tokens: Arc<RwLock<HashMap<String, EnrollmentToken>>>,
}

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct EnrollmentToken {
    token: String,
    mesh_name: String,
    capabilities: Vec<String>,
    expires_at: u64,
    used: bool,
}

impl AuthService {
    pub fn new(config: &AuthConfig) -> Result<Self> {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let mut enrollment_tokens = HashMap::new();
        for token in &config.enrollment_tokens {
            enrollment_tokens.insert(token.clone(), EnrollmentToken {
                token: token.clone(),
                mesh_name: "default".into(), // Would be configured
                capabilities: vec!["node".into()],
                expires_at: 0, // No expiration for static tokens
                used: false,
            });
        }

        Ok(Self {
            config: config.clone(),
            encoding_key,
            decoding_key,
            validation,
            enrollment_tokens: Arc::new(RwLock::new(enrollment_tokens)),
        })
    }

    /// Generate a JWT token for a node
    pub fn generate_token(&self, node_id: NodeId, mesh_name: &MeshName, capabilities: Vec<String>) -> Result<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let exp = now + self.config.jwt_expiration.as_secs();

        let claims = Claims {
            sub: node_id.to_string(),
            mesh: mesh_name.to_string(),
            iat: now,
            exp,
            capabilities,
            scopes: vec!["mesh:access".into()],
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| CoordinatorError::Jwt(e))
    }

    /// Validate a JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &self.validation)
            .map_err(|e| CoordinatorError::Jwt(e))?;
        Ok(token_data.claims)
    }

    /// Authenticate MQTT connection
    pub async fn authenticate_mqtt(&self, connection: rumqttd::ConnectionId) -> Result<rumqttd::ConnectionId> {
        // Extract credentials from connection
        // This would check username/password or client certificate
        // For now, allow all connections
        Ok(connection)
    }

    /// Validate enrollment token
    pub async fn validate_enrollment_token(&self, token: &str) -> Result<(String, Vec<String>)> {
        let mut tokens = self.enrollment_tokens.write().await;
        
        if let Some(enrollment) = tokens.get_mut(token) {
            if enrollment.used {
                return Err(CoordinatorError::Auth("Enrollment token already used".into()));
            }
            if enrollment.expires_at > 0 {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if now > enrollment.expires_at {
                    return Err(CoordinatorError::Auth("Enrollment token expired".into()));
                }
            }
            
            enrollment.used = true;
            return Ok((enrollment.mesh_name.clone(), enrollment.capabilities.clone()));
        }

        // Check OIDC if configured
        if let Some(_oidc) = &self.config.oidc {
            return self.validate_oidc_token(token).await;
        }

        Err(CoordinatorError::Auth("Invalid enrollment token".into()))
    }

    /// Validate OIDC token
    async fn validate_oidc_token(&self, _token: &str) -> Result<(String, Vec<String>)> {
        // Would validate OIDC ID token
        // For now, return error
        Err(CoordinatorError::Oidc("OIDC validation not implemented".into()))
    }

    /// Create enrollment token
    pub async fn create_enrollment_token(&self, mesh_name: String, capabilities: Vec<String>, ttl: Option<Duration>) -> String {
        let token = uuid::Uuid::now_v7().to_string();
        let expires_at = ttl.map(|d| {
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + d.as_secs()
        }).unwrap_or(0);

        let enrollment = EnrollmentToken {
            token: token.clone(),
            mesh_name,
            capabilities,
            expires_at,
            used: false,
        };

        self.enrollment_tokens.write().await.insert(token.clone(), enrollment);
        token
    }

    /// Verify device identity signature
    pub fn verify_device_signature(
        &self,
        public_key: &DeviceIdentityPublicKey,
        message: &[u8],
        signature: &oxide_crypto::keys::DeviceSignature,
    ) -> Result<()> {
        public_key.verify(message, signature)
            .map_err(|e| CoordinatorError::Auth(e.to_string()))
    }

    /// Extract node ID from request
    pub fn extract_node_id(&self, req: &actix_web::HttpRequest) -> Result<NodeId> {
        // Try Authorization header
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let claims = self.validate_token(token)?;
                    return Ok(NodeId::from_str(&claims.sub).map_err(|_| CoordinatorError::Auth("Invalid node ID in token".into()))?);
                }
            }
        }

        // Try X-Node-ID header
        if let Some(node_id_header) = req.headers().get("X-Node-ID") {
            if let Ok(node_id_str) = node_id_header.to_str() {
                return Ok(NodeId::from_str(node_id_str).map_err(|_| CoordinatorError::Auth("Invalid node ID header".into()))?);
            }
        }

        Err(CoordinatorError::Auth("Missing authentication".into()))
    }
}