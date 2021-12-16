use cafeteria::yew::{dir, picture, Gallery, GalleryConfig, GalleryModel};
use presentation::playing::hand::{HandHint, HandHints, HintType};
use presentation::playing::{password_form::PasswordForm,hand::Hand};
use presentation::{
    home::home,
    meeting::{meeting_guest, GuestForm},
    members::Member,
    sleep::sleep,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Config;

impl GalleryConfig for Config {
    fn model() -> GalleryModel {
        GalleryModel::new([
            ("home", picture(|| home(&Callback::noop()))),
            (
                "meeting",
                dir([
                    (
                        "guest",
                        picture(|| {
                            meeting_guest(
                                &GuestForm::Joinable {
                                    join: Callback::noop(),
                                },
                                &vec![
                                    Member {
                                        name: "aaaa".to_string(),
                                        you: true,
                                    },
                                    Member {
                                        name: "iii".to_string(),
                                        you: false,
                                    },
                                ],
                            )
                        }),
                    ),
                    ("host", picture(|| todo!())),
                ]),
            ),
            (
                "playing",
                dir([
                    (
                        "password",
                        picture(|| {
                            html! {
                                <PasswordForm hints_num=3 submit=Callback::noop()/>
                            }
                        }),
                    ),
                    (
                        "select placing",
                        picture(|| {
                            let hints : HandHints = vec![
                                HandHint { text: "あああ".into() , typ: HintType::Target, select: Callback::noop() },
                                HandHint { text: "いいい".into() , typ: HintType::None, select: Callback::noop() },
                                HandHint { text: "ううう".into() , typ: HintType::None, select: Callback::noop() }
                            ];
                            html! {
                                <Hand hints=hints/>
                            }
                        }),
                    ),
                ]),
            ),
            ("sleep", picture(sleep)),
        ])
    }
}

pub fn main() {
    panic!()
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::start_app::<Gallery<Config>>();
}
