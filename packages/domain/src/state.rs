use exprocess::core::ExprocessCore;
use rand::thread_rng;

use crate::{model::BoardState, function::{InitCommand, init}};

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
            (AppState::Blank, AppCommand::Init(command)) => {
                AppResult::Init(init(command, &mut thread_rng()))
            },
            (_, AppCommand::Init(_)) => panic!(),
        }
    }

    fn reducer(mut state: &mut Self::State, result: Self::Result) {
        match (&mut state,result) {
            (AppState::Blank, AppResult::Init(board)) => {
                *state = AppState::Board(board)
            },
            (_, AppResult::Init(_)) => panic!(),
        }
    }
}

pub enum AppCommand {
    Init(InitCommand)
}

pub enum AppState {
    Blank,Board(BoardState)
}

pub enum AppResult {
    Init(BoardState)
}