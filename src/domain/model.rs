use std::collections::HashMap;

pub struct BoardState {
    pub hints: HashMap<HintId,Hint>,
    pub players: HashMap<PlayerId,Player>
}

#[derive(Eq,Hash,Clone,PartialEq)]
pub struct PlayerId(pub usize);

pub struct Player {
    pub name: String,
    pub password: String,
    pub target: PlayerId,
    pub knowledges: PlayerKnowledges
}

pub struct PlayerKnowledges {
    pub target: HintId,
    pub others:  Vec<HintId>
}

#[derive(Eq,Hash,Clone,PartialEq)]
pub struct HintId(pub usize);

pub struct Hint {
    pub text: String,
    pub player: PlayerId
}

