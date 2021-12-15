use crate::{
    domain::{
        repository::RepositoryError, start,Runner,
    },
};

use js_bridge::fetch_members;
use presentation::{loading::loading};
use yew::prelude::*;
mod model;
use crate::containers::main::model::{app_state_to_view_state,ViewState,Msg};

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
                    &link_listener.callback(|e| e)
                );
                link_listener.send_message(Msg::UpdateState(state))
            }),
            Box::new(move |err| match err {
                RepositoryError::UnExpected => link_on_error.emit(())
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
                    let _link = self.link.clone();
                    let on_error =  self.props.on_error.clone();
                    fetch_members(
                        self.props.room_id.as_str(),
                        move |_members| {
                            
                        },
                        move || on_error.clone().emit(())
                    );
                }
                self.state = state
            }
            Msg::PushCommand(command) => self.runner.dispatch(command)
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        match &self.state {
            ViewState::Blank => loading(),
            ViewState::Board(_) => todo!()
        }
    }
}
