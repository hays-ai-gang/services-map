use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_repo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_servers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_clients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<QueueConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_http_server: Option<bool>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_queues: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_queues: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicesMap {
    pub services: Vec<ServiceDefinition>,
}
