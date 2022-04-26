use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GCloudRunEnv {
    #[serde(rename = "gae_application")]
    pub application: String,
    #[serde(rename = "gae_deployment_id")]
    pub deployment_id: String,
    #[serde(rename = "gae_env")]
    pub env: String,
    #[serde(rename = "gae_memory_mb")]
    pub memory_mb: usize,
    #[serde(rename = "gae_runtime")]
    pub runtime: String,
    #[serde(rename = "gae_service")]
    pub service: String,
    #[serde(rename = "gae_version")]
    pub version: String,
    #[serde(rename = "google_cloud_project")]
    pub project: String,
    pub port: u16,
}

impl GCloudRunEnv {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}
