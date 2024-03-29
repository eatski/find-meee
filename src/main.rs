use presentation::layout::layout;
use wasm_bindgen::prelude::*;

use yew::prelude::*;

mod containers;
mod domain;
mod pages;
mod routing;

use pages::{home::Home, room::Room};

use presentation::{error};

use crate::routing::{AppRoute, AppRouter};
use crate::containers::sleeper::Sleeper;

pub enum Msg {
    Error,
}

pub enum State {
    Error,
    Ok,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State::Ok,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Error => {
                self.state = State::Error;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        layout(match self.state {
            State::Error => error::error(),
            State::Ok => {
                let link = self.link.clone();
                let render = AppRouter::render(move |switch: AppRoute| {
                    let on_error = link.callback(|_| Msg::Error);
                    match switch {
                        AppRoute::Home => {
                            html! { 
                                <Home on_error=on_error/> 
                            }
                        }
                        AppRoute::Room(room_id) => {
                            html! { 
                                <Sleeper>
                                    <Room room_id=room_id on_error=on_error/> 
                                </Sleeper>
                            }
                        }
                    }
                });
                html! {
                    <AppRouter
                        render=render
                        redirect=AppRouter::redirect(|_| AppRoute::Home)
                    />
                }
            }
        })
    }
}

#[wasm_bindgen]
pub fn start(mode: AppMode) {
    let log_level = match mode {
        AppMode::Dev => log::Level::Trace,
        AppMode::Production => log::Level::Error,
    };
    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::start_app::<App>();
}

// コンパイルエラー回避のため仕方なく
pub fn main() {
    panic!()
}

#[wasm_bindgen]
pub enum AppMode {
    Dev,
    Production,
}
