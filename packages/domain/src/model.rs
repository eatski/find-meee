use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct BoardState {
    pub hints: HashMap<HintId,Hint>,
    pub players: HashMap<PlayerId,Player>
}

pub type Hints =  HashMap<HintId,Hint>;

#[derive(Eq,Hash,Clone,PartialEq,Debug,Serialize,Deserialize)]
pub struct PlayerId(pub usize);

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Player {
    pub name: String,
    pub password: String,
    pub hints: Vec<HintId>,
    pub target: PlayerId,
    pub knowledges: PlayerKnowledges
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct PlayerKnowledges {
    pub target: HintId,
    pub others:  Vec<HintId>
}

#[derive(Eq,Hash,Clone,PartialEq,Debug,Serialize,Deserialize)]
pub struct HintId(pub usize);

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Hint {
    pub text: String
}
