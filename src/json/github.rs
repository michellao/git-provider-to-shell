use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Package {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct GithubPackageWebhook {
    pub action: String,
    pub package: Package,
}
