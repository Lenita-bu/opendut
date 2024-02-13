#[derive(thiserror::Error, Clone, Debug)]
pub(crate) enum TheoError {
    #[error("ConsumeOutputError Error: {0}")]
    ConsumeOutputError(String),
    #[error("Docker command failed: {0}")]
    DockerCommandFailed(String),
    #[error("Timeout: {0}")]
    Timeout(String),
}

pub(crate) const TARGET_TRIPLE: &str = "x86_64-unknown-linux-gnu";
pub(crate) const OPENDUT_REPO_ROOT: &str = "OPENDUT_REPO_ROOT";
pub(crate) const OPENDUT_EXPOSE_PORTS: &str = "OPENDUT_EXPOSE_PORTS";
pub(crate) const OPENDUT_VM_NAME: &str = "opendut-vm";
pub(crate) const TIMEOUT_SECONDS: u64 = 300;
pub(crate) const SLEEP_TIME_SECONDS: u64 = 5;

pub type Result = anyhow::Result<()>;

pub(crate) mod project;
pub(crate) mod docker;
pub(crate) mod metadata;
pub(crate) mod util;
pub(crate) mod dist;

pub(crate) mod network;
pub(crate) mod command_ext;
pub(crate) mod carl_config;
