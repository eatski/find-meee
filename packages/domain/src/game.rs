use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    libs::simultaneously::Simulataneously,
    model::{HintId, Player, PlayerId},
};

#[derive(Clone)]
pub enum GamePhase {
    SelectPutToMarket(Simulataneously<PlayerId, PutToMarket>),
    SelectAction(Market, Simulataneously<PlayerId, Action>),
    ConfirmAnswer(Vec<(PlayerId, Answer)>),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum PutToMarket {
    Hint(HintId),
    Coin,
}

pub type Market = HashMap<PlayerId, PutToMarket>;

#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
    Exchange(PlayerId),
    Answer(Answer),
    Pass,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameCommand {
    PushPutToMarket(PlayerId, PutToMarket),
    PushAction(PlayerId, Action),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Answer {
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameResult {
    PushPutToMarket(PlayerId, PutToMarket),
    MoveToActionPhase(Market),
    PushAction(PlayerId, Action),
    MoveToNext(ActionResult, Vec<(PlayerId, Answer)>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ActionResult {
    pub players: HashMap<PlayerId, PlayerHintMove>,
    pub ansers: Vec<(PlayerId, Answer)>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerHintMove {
    pub refs: Vec<HintId>,
    pub ownership: Vec<HintId>,
    pub loss_ownership: Vec<HintId>,
    pub move_coin: Vec<CoinMove>,
}
pub struct ExchangeSuccess {
    pub target: PlayerId,
    pub hint: HintId,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CoinMove {
    Get,Loss
}

pub fn resolve_actions(
    players: &Vec<PlayerId>,
    market: Market,
    actions: Vec<(PlayerId, Action)>,
) -> ActionResult {
    let mut results: HashMap<_, _> = players
        .iter()
        .map(|p| {
            (
                p.clone(),
                PlayerHintMove {
                    refs: Vec::new(),
                    ownership: Vec::new(),
                    loss_ownership: Vec::new(),
                    move_coin: Vec::new()
                },
            )
        })
        .collect();
    let mut ansers = Vec::new();
    for (player, action) in actions.into_iter() {
        match action {
            Action::Exchange(target) => {
                let player_put =  market.get(&player).expect("TODO");
                let target_put = market.get(&target).expect("TODO");
                let mut result = results.remove(&player).expect("TODO");
                let mut target_result = results.remove(&target).expect("TODO");
                match target_put {
                    PutToMarket::Hint(hint) => {
                        result.refs.push(hint.clone());
                        match player_put {
                            PutToMarket::Hint(hint) => {
                                target_result.ownership.push(hint.clone());
                                result.loss_ownership.push(hint.clone());
                            },
                            PutToMarket::Coin => {
                                target_result.move_coin.push(CoinMove::Get);
                                result.move_coin.push(CoinMove::Loss);
                            },
                        }
                    }
                    PutToMarket::Coin => panic!("コインは選択できない"),
                }
                results.insert(player, result);
                results.insert(target, target_result);
            }
            Action::Answer(answer) => ansers.push((player, answer)),
            Action::Pass => { /* noop */ }
        }
    }
    ActionResult {
        players: results,
        ansers,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_resolve_actions() {
        unimplemented!();
    }
}
