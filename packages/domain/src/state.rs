use core::panic;

use exprocess::core::ExprocessCore;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::{
    init::{init, InitBoard, InitPlayer},
    libs::simultaneously::Simulataneously,
    model::{BoardState, PlayerId},
    profile::Profiles,
    setting::Setting, game::{GamePhase, GameCommand, GameResult},
};

pub struct AppCore;

impl ExprocessCore for AppCore {
    type State = AppState;

    type Command = AppCommand;

    type Result = AppResult;

    fn init() -> Self::State {
        AppState::Blank
    }

    fn resolve(state: &Self::State, command: Self::Command) -> Self::Result {
        match (state, command) {
            (AppState::Blank, AppCommand::InitProfile(profiles)) => {
                AppResult::InitProfile(profiles)
            }
            (AppState::Blank, _) => panic!(),
            (AppState::StandbyPassword(_, inputs, setting), AppCommand::PushPassword(id,player)) => {
                match inputs.is_complete() {
                    Some(complete) => {
                        let board = init(
                            InitBoard {
                                players: complete.finalize(id,player),
                                hints_num: setting.hints_num,
                            },
                            &mut thread_rng(),
                        );
                        AppResult::InitBoard(board)
                    }
                    None => AppResult::PushPassword(id,player),
                }
            }
            (AppState::StandbyPassword(_, _, _), _) => panic!(),
            (AppState::Board(_, _, phase), AppCommand::Board(command)) => match (phase, command) {
                (
                    GamePhase::SelectAction(_,inputs),
                    GameCommand::PushPutToMarket(id, input),
                ) => AppResult::Board(match inputs.is_complete() {
                    Some(complete) => {
                        todo!()
                    }
                    None => GameResult::PushPutToMarket(id,input),
                }),
                (GamePhase::SelectPutToMarket(_), GameCommand::PushPutToMarket(_, _)) => todo!(),
                (GamePhase::ConfirmAnswer(_), GameCommand::PushPutToMarket(_, _)) => todo!(),
                (GamePhase::SelectPutToMarket(_), GameCommand::PushAction(_, _)) => todo!(),
                (GamePhase::SelectAction(_, _), GameCommand::PushAction(_, _)) => todo!(),
                (GamePhase::ConfirmAnswer(_), GameCommand::PushAction(_, _)) => todo!(),
               
            },
            (AppState::Board(_board, _profiles, _), _) => panic!(),
        }
    }

    fn reducer(mut state: &mut Self::State, result: Self::Result) {
        match (&mut state, result) {
            (AppState::Blank, AppResult::InitProfile(profiles)) => {
                let len = profiles.players.len();
                *state = AppState::StandbyPassword(
                    profiles,
                    Simulataneously::new(len),
                    Setting::recommend(),
                );
            }
            (AppState::Blank, _) => panic!(),
            (AppState::StandbyPassword(profiles, _, _), AppResult::InitBoard(board)) => {
                *state = AppState::Board(
                    board,
                    profiles.clone(),
                    GamePhase::SelectAction(todo!(),profiles.players.len().into()),
                );
            }
            (AppState::StandbyPassword(_, inputs, _), AppResult::PushPassword(id,player)) => {
                inputs.insert(id,player);
            }
            (AppState::StandbyPassword(_, _, _), _) => panic!(),
            (AppState::Board(state, profiles, phase), AppResult::Board(result)) => {
                
                match (phase.clone(),result) {
                    (GamePhase::SelectPutToMarket(mut inputs), GameResult::PushPutToMarket(id, input)) => {
                        inputs.insert(id, input);
                    },
                    (GamePhase::SelectPutToMarket(inputs),GameResult::MoveToActionPhase(market)) => {
                        GamePhase::SelectAction(market,profiles.players.len().into());
                        *phase = GamePhase::ConfirmAnswer(Vec::new());
                    },
                    (GamePhase::ConfirmAnswer(_), GameResult::PushPutToMarket(_, _)) => todo!(),
                    _ => panic!()
                };
                
                
            },
            (AppState::Board(_, _, _), _) => panic!(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppCommand {
    InitProfile(Profiles),
    PushPassword(PlayerId,InitPlayer),
    Board(GameCommand),
}


pub enum AppState {
    Blank,
    StandbyPassword(Profiles, Simulataneously<PlayerId,InitPlayer>, Setting),
    Board(BoardState, Profiles, GamePhase),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AppResult {
    InitProfile(Profiles),
    PushPassword(PlayerId,InitPlayer),
    InitBoard(BoardState),
    Board(GameResult),
}
