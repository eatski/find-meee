use std::iter::repeat;

use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties};

pub struct PasswordForm {
    pub form: Form,
    pub link: ComponentLink<Self>,
    pub props: Props
}

pub enum Msg {
    ChangePassword(String),
    ChangeHint(usize, String),
}

#[derive(Properties, Clone)]
pub struct Props {
    pub hints_num: usize,
    pub submit: Callback<Form>,
}

#[derive(Clone)]
pub struct Form {
    pub password: String,
    pub hints: Vec<String>,
}

impl Component for PasswordForm {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            form: Form {
                password: String::new(),
                hints: repeat(String::new()).take(props.hints_num).collect(),
            },
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::ChangePassword(password) => {
                self.form.password = password;
            }
            Msg::ChangeHint(index, hint) => {
                *self.form.hints.get_mut(index).expect("TODO") = hint;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let on_password_change = self
            .link
            .callback(|input: InputData| Msg::ChangePassword(input.value));
        let link = self.link.clone();
        let hint_forms = self.form.hints.iter().enumerate().map(|(index, hint)| {
            hint_form(
                hint.clone(),
                link.callback(move |val| Msg::ChangeHint(index, val)),
            )
        });
        let form = self.form.clone();
        let on_submit = self.props.submit.reform(move |_| form.clone());
        html! {
            <div class="column">
                <div class="field">
                    <label class="label">{"あいことば"}</label>
                    <div class="control">
                        <input class="input" type="text" value=self.form.password.clone() oninput=on_password_change />
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"ひんと"}</label>
                    <ul>
                        {for hint_forms}
                    </ul>
                </div>
                <div class="field">
                    <div class="control">
                        <button onclick=on_submit class="button is-link">{"決定"}</button>
                    </div>
                </div>
            </div>
        }
    }
}

fn hint_form(hint: String, callback: Callback<String>) -> Html {
    let on_change = callback.reform(|input: InputData| input.value);
    html! {
        <li class="control field">
            <input class="input" type="text" value=hint oninput=on_change />
        </li>
    }
}
