use crate::config::sources::default_true;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocalSourceConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
}

impl Default for LocalSourceConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}
