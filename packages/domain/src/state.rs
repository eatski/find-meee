use core::panic;

use exprocess::core::ExprocessCore;
use rand::thread_rng;
use serde::{Serialize, Deserialize};

use crate::{model::{BoardState}, function::{InitBoard, init, InitPlayer}, profile::{Profiles}};

pub struct AppCore;

impl ExprocessCore for AppCore {
    type State = AppState;

    type Command = AppCommand;

    type Result = AppResult;

    fn init() -> Self::State {
        AppState::Blank
    }

    fn resolve(state: &Self::State, command: Self::Command) -> Self::Result {
        match (state,command) {
            (AppState::Blank, AppCommand::InitProfile(profiles)) => AppResult::InitProfile(profiles),
            (AppState::Blank, _) => panic!(),
            (AppState::StandbyPassword(profiles,inputs), AppCommand::PushPassword(input)) => {
                if inputs.len() + 1 < profiles.players.len() {
                    return AppResult::PushPassword(input)
                }
                let inputs: Vec<_> = inputs.iter().cloned().chain([input].into_iter()).collect();
                let board = init(InitBoard {
                    players: inputs,
                    hints_num: 3,
                },&mut thread_rng());
                AppResult::InitBoard(board)
            },
            (AppState::StandbyPassword(_, _), _) => panic!(),
            (AppState::Board(_board,_profiles), AppCommand::InitProfile(_)) => todo!(),
            (AppState::Board(_, _), AppCommand::PushPassword(_)) => todo!(),
        }
    }

    fn reducer(mut state: &mut Self::State, result: Self::Result) {
        match (&mut state,result) {
            (AppState::Blank, AppResult::InitProfile(profiles)) => {
                let len = profiles.players.len();
                *state = AppState::StandbyPassword(profiles,Vec::with_capacity(len));
            },
            (AppState::Blank,_) => panic!(),
            (AppState::StandbyPassword(profiles,_), AppResult::InitBoard(board)) => {
                *state = AppState::Board(board,profiles.clone());
            },
            (AppState::StandbyPassword(_,inputs), AppResult::PushPassword(input)) => {
                inputs.push(input);
            },
            (AppState::StandbyPassword(_,_), _) => panic!() ,
            (AppState::Board(_, _), AppResult::InitProfile(_)) => todo!(),
            (AppState::Board(_, _), AppResult::PushPassword(_)) => todo!(),
            (AppState::Board(_, _), AppResult::InitBoard(_)) => todo!(),
        }
    }
}

#[derive(Serialize,Deserialize,Clone)]
pub enum AppCommand {
    InitProfile(Profiles),
    PushPassword(InitPlayer)
}

pub enum AppState {
    Blank,
    StandbyPassword(Profiles,Vec<InitPlayer>),
    Board(BoardState,Profiles)
}

#[derive(Serialize,Deserialize,Clone)]
pub enum AppResult {
    InitProfile(Profiles),
    PushPassword(InitPlayer),
    InitBoard(BoardState)
}

