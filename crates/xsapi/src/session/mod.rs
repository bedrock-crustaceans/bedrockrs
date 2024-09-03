use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub properties: SessionProperties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionProperties {
    pub system: SessionPropertiesSystem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionPropertiesSystem {
    #[serde(rename = "joinRestriction")]
    pub join_restriction: String,
    #[serde(rename = "readRestriction")]
    pub read_restriction: String,
    pub closed: bool,
}
