//! Configuration for TLS management.

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use tracing::{event, Level};

/// Configuration for TLS management.
#[derive(Debug)]
pub struct TlsConfig {
    /// Paths for TLS files.
    paths: TlsPaths,

    /// Whether keypair and certificate can be generated for TLS.
    can_generate_secrets: bool,
}

/// Directory and file paths for storing TLS management data.
#[derive(Debug)]
pub struct TlsPaths {
    /// Directory that contains the data for TLS.
    base_dir: PathBuf,

    /// File that contains the certificate.
    cert_file: PathBuf,

    /// File that contains the private key.
    private_key_file: PathBuf,
}

impl TlsConfig {
    /// Create configuration for TLS management.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory for storing TLS data.
    /// * `can_generate_secrets` - Whether to automatically generate keypair and certificate for TLS if
    ///                            not present in the data directory.
    pub fn new(base_dir: &Path, can_generate_secrets: bool) -> Self {
        Self {
            paths: TlsPaths::new(base_dir),
            can_generate_secrets,
        }
    }

    /// Prepare the data directory for TLS.
    ///
    /// # Errors
    ///
    /// * The directory could not be created.
    pub fn prepare_directory(&self) -> Result<()> {
        self.paths.prepare_directory()?;

        Ok(())
    }

    /// Get the paths for TLS files and directories.
    pub fn paths(&self) -> &TlsPaths {
        &self.paths
    }

    /// Get whether TLS secret generation is allowed.
    pub fn can_generate_secrets(&self) -> bool {
        self.can_generate_secrets
    }
}

impl TlsPaths {
    /// Create path configuration for TLS management.
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory for storing TLS data.
    pub fn new(base_dir: &Path) -> Self {
        let base_dir = base_dir.to_path_buf();

        Self {
            cert_file: base_dir.join("scheduler.pem"),
            private_key_file: base_dir.join("scheduler.key"),
            base_dir,
        }
    }

    /// Prepare the data directory for TLS.
    ///
    /// # Errors
    ///
    /// * The directory could not be created.
    pub fn prepare_directory(&self) -> Result<()> {
        // Create the base directory
        match fs::create_dir_all(&self.base_dir) {
            Err(e) if e.kind() != ErrorKind::AlreadyExists => Err(e),
            _ => Ok(()),
        }
        .with_context(|| {
            format!(
                "Unable to create the base TLS directory: {}",
                self.base_dir.display()
            )
        })?;

        event!(Level::DEBUG, path = %self.base_dir.display(), "Created TLS base directory");
        Ok(())
    }

    /// Get the path to the X509 certificate file.
    pub fn cert_file(&self) -> &PathBuf {
        &self.cert_file
    }

    /// Get the path to the private key file.
    pub fn private_key_file(&self) -> &PathBuf {
        &self.private_key_file
    }
}
