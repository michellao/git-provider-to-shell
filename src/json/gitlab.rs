use serde::{Deserialize, Serialize};

use crate::core::common::EventType;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    pub default_branch: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct GitlabWebhook {
    pub object_kind: EventType,
    pub project: Project,
}
