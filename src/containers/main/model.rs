use yew::prelude::*;

use domain::{state::{AppState, AppCommand}, model::BoardState};

pub enum ViewState {
    Blank,
    Board(BoardState)
}

pub fn app_state_to_view_state(app: &AppState, is_host: bool, your_id: &str,callback: &Callback<Msg>) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Board(board,_) => ViewState::Board(board.clone()),
        AppState::StandbyPassword(_, _) => todo!(),
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}