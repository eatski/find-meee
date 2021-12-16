use presentation::playing::{
    hand::{HandHints, HintType},
    password_form::Form as PasswordForm,
};
use yew::prelude::*;

use domain::{
    init::InitPlayer,
    model::HintId,
    state::{AppCommand, AppState, Setting},
};

pub enum ViewState {
    Blank,
    InputPassword(Callback<PasswordForm>, Setting),
    Board(BoardView),
    TODO(String),
}

pub enum BoardView {
    SelectPlacingHint { hints: HandHints },
}

pub fn app_state_to_view_state(
    app: &AppState,
    _is_host: bool,
    your_id: &str,
    callback: &Callback<Msg>,
) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Board(board, profiles) => {
            let profile = profiles.players.get(your_id).expect("TODO");
            let player = board.players.get(&profile.id).expect("TODO");
            let get_hint = |id: &HintId| board.hints.get(id).expect("TODO");
            let hints = player
                .knowledges
                .others
                .iter()
                .map(|hint| (get_hint(hint).text.clone(), HintType::None))
                .chain([(
                    get_hint(&player.knowledges.target).text.clone(),
                    HintType::Target,
                )])
                .collect();
            ViewState::Board(BoardView::SelectPlacingHint { hints })
        }
        AppState::StandbyPassword(profiles, inputs, setting) => {
            let player = profiles.players.get(your_id).expect("TODO");
            let complete = inputs.iter().find(|input| input.id == player.id);
            if let Some(complete) = complete {
                ViewState::TODO(serde_json::to_string(complete).expect("TODO"))
            } else {
                let id = player.id.clone();
                let callback = callback.reform(move |form: PasswordForm| {
                    Msg::PushCommand(AppCommand::PushPassword(InitPlayer {
                        id: id.clone(),
                        password: form.password,
                        hints: form.hints,
                    }))
                });
                ViewState::InputPassword(callback, setting.clone())
            }
        }
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}
