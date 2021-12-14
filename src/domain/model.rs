use std::collections::HashMap;

#[derive(Debug)]
pub struct BoardState {
    pub hints: HashMap<HintId,Hint>,
    pub players: HashMap<PlayerId,Player>
}

pub type Hints =  HashMap<HintId,Hint>;

#[derive(Eq,Hash,Clone,PartialEq,Debug)]
pub struct PlayerId(pub usize);

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub password: String,
    pub hints: Vec<HintId>,
    pub target: PlayerId,
    pub knowledges: PlayerKnowledges
}

#[derive(Debug)]
pub struct PlayerKnowledges {
    pub target: HintId,
    pub others:  Vec<HintId>
}

#[derive(Eq,Hash,Clone,PartialEq,Debug)]
pub struct HintId(pub usize);

#[derive(Debug)]
pub struct Hint {
    pub text: String
}
