use crate::domain::{repository::RepositoryError, start, Runner};

use domain::{
    model::PlayerId,
    profile::{PlayerProfile, Profiles},
    state::AppCommand,
};
use js_bridge::fetch_members;
use presentation::{loading::loading, playing::hand::Hand};
use yew::prelude::*;
mod model;
use crate::containers::main::model::{app_state_to_view_state, Msg, ViewState};
use presentation::playing::password_form::PasswordForm;

pub struct Main {
    runner: Runner,
    state: ViewState,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub is_host: bool,
    pub room_id: String,
    pub your_id: String,
    pub on_error: Callback<()>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_listener = link.clone();
        let link_on_error = props.on_error.clone();
        let is_host = props.is_host;
        let your_id = props.your_id.clone();
        let runner = start(
            props.room_id.clone(),
            Box::new(move |_, state| {
                let state = app_state_to_view_state(
                    &state,
                    is_host,
                    your_id.as_str(),
                    &link_listener.callback(|e| e),
                );
                link_listener.send_message(Msg::UpdateState(state))
            }),
            Box::new(move |err| match err {
                RepositoryError::UnExpected => link_on_error.emit(()),
            }),
        );
        Main {
            state: ViewState::Blank,
            runner,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateState(state) => {
                if matches!(state, ViewState::Blank) && self.props.is_host {
                    let link = self.link.clone();
                    let on_error = self.props.on_error.clone();
                    fetch_members(
                        self.props.room_id.as_str(),
                        move |members| {
                            let profiles = Profiles {
                                players: members
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, member)| {
                                        (
                                            member.id.to_string(),
                                            PlayerProfile {
                                                id: PlayerId(index),
                                                display_name: member.name.to_string(),
                                            },
                                        )
                                    })
                                    .collect(),
                            };
                            let command = AppCommand::InitProfile(profiles);
                            link.send_message(Msg::PushCommand(command))
                        },
                        move || on_error.clone().emit(()),
                    );
                }
                self.state = state
            }
            Msg::PushCommand(command) => self.runner.dispatch(command),
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        match &self.state {
            ViewState::Blank => loading(),
            ViewState::Board(board) => {
                match board {
                    model::BoardView::SelectPlacingHint { hints } => html! { <Hand hints=hints.clone()/>},
                }
            },
            ViewState::TODO(json ) => html! {json},
            ViewState::InputPassword(callback,settings) => html! {<PasswordForm submit=callback hints_num=settings.hints_num/>},
        }
    }
}
