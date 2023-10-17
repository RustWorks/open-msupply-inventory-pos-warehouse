pub(crate) const MANIFEST_FILE: &str = "manifest.txt";
pub(crate) const MANIFEST_SIGNATURE_FILE: &str = "manifest.signature";
pub(crate) const SIGNATURE_TAG: &str = "SIGNATURE";
pub(crate) const CERTIFICATE_TAG: &str = "CERTIFICATE";
pub(crate) const PRIVATE_KEY_TAG: &str = "PRIVATE KEY";

pub(crate) const SHA256_NAME: &str = "sha-256";
pub(crate) const VERIFICATION_ALGO_PSS: &str = "pss";

pub(crate) const PLUGIN_FILE_DIR: &'static str = "plugins";

pub mod manifest;
pub mod plugin_files;
pub mod validation;
