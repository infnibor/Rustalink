use crate::config::sources::default_true;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpSourceConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for HttpSourceConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}
