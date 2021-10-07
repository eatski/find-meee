use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

mod javascript;
mod repository;
mod components;
mod pages;
use pages::{
    home::Home, page_not_found::PageNotFound,room::Room,
};
mod switch;
use switch::{AppRoute, AppRouter, PublicUrlSwitch};

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <AppRouter
                    render=AppRouter::render(Self::switch)
                    redirect=AppRouter::redirect(|route: Route| {
                        AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                    })
                />
            </main>
        }
    }
}
impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
            AppRoute::Room(id) => {
                html! { <Room id=id/> }
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
