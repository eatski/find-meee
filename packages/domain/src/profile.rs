use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::model::PlayerId;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct PlayerProfile {
    pub display_name: String,
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Profiles {
    pub players: HashMap<PlayerId,PlayerProfile>
}
