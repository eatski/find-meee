use yew::prelude::*;

use domain::{state::{AppState, AppCommand}, model::BoardState};

pub enum ViewState {
    Blank,
    Board(BoardState),
    TODO(String)
}

pub fn app_state_to_view_state(app: &AppState, is_host: bool, your_id: &str,callback: &Callback<Msg>) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Board(board,_) => ViewState::Board(board.clone()),
        AppState::StandbyPassword(profiles, _) => {
           let player =  profiles.players.get(your_id).expect("TODO");
           ViewState::TODO(serde_json::to_string(player).expect("TODO"))
        },
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}