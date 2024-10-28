use serde::{Deserialize, Serialize};

// TODO: Figure out what fields are optional and which are not
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityInfo {
    /// Refers to the RuntimeID.
    rid: i32,
    id: String,
    /// Refers to the BaseID.
    // TODO: Find out what a BaseID id is
    bid: String,
    #[serde(rename = "hasspawnegg")]
    has_spawn_egg: bool,
    summonable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityInfoList {
    list: Vec<EntityInfo>,
}
