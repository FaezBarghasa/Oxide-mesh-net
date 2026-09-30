//! Systemd-resolved integration for automatic link-level DNS management

use std::process::Command;
use tracing::{debug, error, info, warn};

/// Systemd-resolved link DNS configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdResolvedConfig {
    pub interface: String,
    pub dns_servers: Vec<String>,
    pub routing_domains: Vec<String>,
    pub dry_run: bool,
}

impl Default for SystemdResolvedConfig {
    fn default() -> Self {
        Self {
            interface: "oxide0".to_string(),
            dns_servers: vec!["100.100.100.100".to_string()],
            routing_domains: vec!["~mesh.oxide".to_string(), "~oxide".to_string()],
            dry_run: false,
        }
    }
}

/// Systemd-resolved controller
pub struct SystemdResolvedController {
    config: SystemdResolvedConfig,
    is_applied: bool,
}

impl SystemdResolvedController {
    pub fn new(config: SystemdResolvedConfig) -> Self {
        Self {
            config,
            is_applied: false,
        }
    }

    /// Check if systemd-resolved is active on the host
    pub fn is_available() -> bool {
        #[cfg(target_os = "linux")]
        {
            std::path::Path::new("/run/systemd/resolve/stub-resolv.conf").exists()
                || std::path::Path::new("/run/systemd/resolve/resolv.conf").exists()
        }
        #[cfg(not(target_os = "linux"))]
        {
            false
        }
    }

    /// Apply DNS configuration via `resolvectl`
    pub fn apply(&mut self) -> Result<(), String> {
        info!(
            "Configuring systemd-resolved on interface {} with DNS {:?} and domains {:?}...",
            self.config.interface, self.config.dns_servers, self.config.routing_domains
        );

        if self.config.dry_run {
            debug!("[DRY-RUN] resolvectl dns {} {:?}", self.config.interface, self.config.dns_servers);
            debug!("[DRY-RUN] resolvectl domain {} {:?}", self.config.interface, self.config.routing_domains);
            self.is_applied = true;
            return Ok(());
        }

        // 1. Set DNS servers on link
        let mut dns_args = vec!["dns", &self.config.interface];
        for s in &self.config.dns_servers {
            dns_args.push(s.as_str());
        }

        let output = Command::new("resolvectl")
            .args(&dns_args)
            .output()
            .map_err(|e| format!("Failed to execute 'resolvectl dns': {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("resolvectl dns error: {}", stderr);
        }

        // 2. Set search / routing domains on link
        let mut domain_args = vec!["domain", &self.config.interface];
        for d in &self.config.routing_domains {
            domain_args.push(d.as_str());
        }

        let output = Command::new("resolvectl")
            .args(&domain_args)
            .output()
            .map_err(|e| format!("Failed to execute 'resolvectl domain': {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("resolvectl domain error: {}", stderr);
        }

        // 3. Set default route link option if exit node enabled (~. domain)
        if self.config.routing_domains.contains(&"~.".to_string()) {
            let _ = Command::new("resolvectl")
                .args(["default-route", &self.config.interface, "yes"])
                .output();
        }

        self.is_applied = true;
        Ok(())
    }

    /// Revert DNS configuration on interface
    pub fn revert(&mut self) -> Result<(), String> {
        if !self.is_applied {
            return Ok(());
        }

        info!("Reverting systemd-resolved DNS settings for interface {}...", self.config.interface);
        if self.config.dry_run {
            debug!("[DRY-RUN] resolvectl revert {}", self.config.interface);
            self.is_applied = false;
            return Ok(());
        }

        let output = Command::new("resolvectl")
            .args(["revert", &self.config.interface])
            .output()
            .map_err(|e| format!("Failed to execute 'resolvectl revert': {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("resolvectl revert error: {}", stderr);
        }

        self.is_applied = false;
        Ok(())
    }

    pub fn is_applied(&self) -> bool {
        self.is_applied
    }
}

impl Drop for SystemdResolvedController {
    fn drop(&mut self) {
        if self.is_applied {
            let _ = self.revert();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_resolved_controller_dry_run() {
        let config = SystemdResolvedConfig {
            interface: "oxide-test0".to_string(),
            dns_servers: vec!["100.100.100.100".to_string()],
            routing_domains: vec!["~mesh.oxide".to_string(), "~.".to_string()],
            dry_run: true,
        };

        let mut controller = SystemdResolvedController::new(config);
        assert!(!controller.is_applied());

        assert!(controller.apply().is_ok());
        assert!(controller.is_applied());

        assert!(controller.revert().is_ok());
        assert!(!controller.is_applied());
    }
}
