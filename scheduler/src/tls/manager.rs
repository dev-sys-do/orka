//! TLS management functions.

use std::{fs::File, io::Write};

use anyhow::{Context, Result};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType};
use time::{Duration, OffsetDateTime};
use tracing::{event, Level};

use super::config::TlsConfig;

/// Manager for TLS secrets.
pub struct TlsManager {
    /// Configuration of the manager.
    config: TlsConfig,

    /// X509 certificate data.
    cert_data: Option<String>,

    /// Private key data.
    key_data: Option<String>,
}

impl TlsManager {
    /// Create a TLS manager.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for this manager.
    pub fn new(config: TlsConfig) -> Self {
        Self {
            config,
            cert_data: None,
            key_data: None,
        }
    }

    /// Populate the certificate and private key data by reading them from the files specified in
    /// the config file, or generating them if these files do not exist and the configuration
    /// allows it.
    ///
    /// # Errors
    ///
    /// * The TLS secrets could not be read from the disk.
    /// * The TLS secrets could not be generated.
    /// * The TLS secrets could not be written to the disk.
    pub fn populate_secrets(&mut self) -> Result<()> {
        // First, try to read the certificate and private key from the disk
        match self.read_secrets() {
            Ok(_) => return Ok(()),
            Err(e) => {
                event!(Level::DEBUG, "Failed to read TLS secrets from disk");

                // If TLS secret generation is disabled, there's nothing more we can do
                if !self.config.can_generate_secrets() {
                    return Err(e).with_context(|| "Unable to read the TLS secrets from the disk");
                }
            }
        }

        // If we weren't successful in reading the secrets from the disk, try generating them
        event!(
            Level::INFO,
            "Generating certificate and private key for TLS"
        );

        self.generate_secrets()
            .with_context(|| "Unable to generate the TLS secrets")?;

        self.write_secrets()
            .with_context(|| "Unable to write the TLS secrets to the disk")?;

        Ok(())
    }

    /// Read the certificate and private key data from the disk. The file paths are specified in
    /// the configuration.
    ///
    /// # Errors
    ///
    /// * The certificate or private key file could not be read.
    fn read_secrets(&mut self) -> Result<()> {
        let tls_paths = self.config.paths();
        let cert_file_path = tls_paths.cert_file();
        let private_key_file_path = tls_paths.private_key_file();

        event!(
            Level::DEBUG,
            path = %cert_file_path.display(),
            "Reading certificate file for TLS"
        );
        let cert_data = std::fs::read_to_string(cert_file_path).with_context(|| {
            format!(
                "Unable to read certificate file for TLS: {}",
                cert_file_path.display()
            )
        })?;

        event!(
            Level::DEBUG,
            path = %private_key_file_path.display(),
            "Reading key file for TLS"
        );
        let key_data = std::fs::read_to_string(private_key_file_path).with_context(|| {
            format!(
                "Unable to read private key file for TLS: {}",
                private_key_file_path.display()
            )
        })?;

        // Everything went correctly, commit the data that was read
        self.cert_data = Some(cert_data);
        self.key_data = Some(key_data);

        Ok(())
    }

    /// Generate a new private key and a new certificate.
    ///
    /// # Errors
    ///
    /// * The self-signed certificate could not be generated.
    /// * The self-signed certificate could not be serialized.
    fn generate_secrets(&mut self) -> Result<()> {
        let subject_alt_names = ["localhost".to_string()];
        let mut cert_params = CertificateParams::new(subject_alt_names);

        let mut dn = DistinguishedName::new();
        dn.push(DnType::OrganizationName, "Orka");
        dn.push(DnType::OrganizationalUnitName, "Scheduler");
        dn.push(DnType::CommonName, "Orka Scheduler TLS certificate");
        cert_params.distinguished_name = dn;

        cert_params.not_before = OffsetDateTime::now_utc() - Duration::days(1);
        cert_params.not_after = OffsetDateTime::now_utc() + Duration::days(365 * 10);

        let cert = Certificate::from_params(cert_params)
            .with_context(|| "Unable to generate the self-signed certificate")?;

        let cert_data = cert
            .serialize_pem()
            .with_context(|| "Unable to serialize the self-signed certificate")?;

        let key_data = cert.serialize_private_key_pem();

        // Everything went correctly, commit the data that was read
        self.cert_data = Some(cert_data);
        self.key_data = Some(key_data);

        Ok(())
    }

    /// Write the certificate and private key data to the disk. The file paths are specified in the
    /// configuration.
    ///
    /// # Errors
    ///
    /// * The certificate or private key data does not exist.
    /// * The certificate or private key file could not be opened (for example, not enough
    ///   permissions or the file already exists).
    /// * The certificate or private key file could not be written to.
    /// * The certificate or private key file could not be synced to the disk.
    fn write_secrets(&mut self) -> Result<()> {
        // Make sure the secrets are here
        let cert_data = self
            .cert_data
            .as_ref()
            .with_context(|| "No certificate data was provided to write to the disk")?;

        let key_data = self
            .key_data
            .as_ref()
            .with_context(|| "No private key data was provided to write to the disk")?;

        // Intentionally prevent overwriting files, because we don't want users to loose data that
        // they might not have saved
        let mut file_opts = File::options();
        file_opts.read(true).write(true).create_new(true);

        let tls_paths = self.config.paths();
        let cert_file_path = tls_paths.cert_file();
        let private_key_file_path = tls_paths.private_key_file();

        event!(
            Level::DEBUG,
            path = %cert_file_path.display(),
            "Writing certificate file for TLS"
        );
        let mut cert_file = file_opts.open(cert_file_path).with_context(|| {
            format!(
                "Unable to open certificate file for TLS: {}",
                cert_file_path.display()
            )
        })?;

        cert_file.write_all(cert_data.as_bytes()).with_context(|| {
            format!(
                "Unable to write certificate file for TLS: {}",
                cert_file_path.display()
            )
        })?;

        event!(
            Level::DEBUG,
            path = %private_key_file_path.display(),
            "Writing key file for TLS"
        );
        let mut private_key_file = file_opts.open(private_key_file_path).with_context(|| {
            format!(
                "Unable to open private key file for TLS: {}",
                private_key_file_path.display()
            )
        })?;

        private_key_file
            .write_all(key_data.as_bytes())
            .with_context(|| {
                format!(
                    "Unable to write private key file for TLS: {}",
                    private_key_file_path.display()
                )
            })?;

        // Make sure the data has reached the disk
        cert_file.sync_data().with_context(|| {
            format!(
                "Unable to sync certificate file to disk for TLS: {}",
                cert_file_path.display()
            )
        })?;

        private_key_file.sync_data().with_context(|| {
            format!(
                "Unable to sync private key file to disk for TLS: {}",
                private_key_file_path.display()
            )
        })?;

        Ok(())
    }

    /// Get the raw data of the X509 certificate.
    pub fn cert_data(&self) -> Option<&String> {
        self.cert_data.as_ref()
    }

    /// Get the raw data of the private key.
    pub fn key_data(&self) -> Option<&String> {
        self.key_data.as_ref()
    }
}
