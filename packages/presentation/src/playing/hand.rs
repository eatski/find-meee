use yew::{Component, Properties, html, Callback};

pub struct Hand {
    props: Props
}

#[derive(Properties,Clone)]
pub struct Props {
    pub hints: HandHints
}

pub type HandHints = Vec<HandHint>;

#[derive(Clone)]
pub struct HandHint {
    pub text: String,
    pub typ: HintType,
    pub select: Callback<()>
}

#[derive(Debug,Clone)]
pub enum HintType {
    None,Target
}

impl Component for Hand {
    type Message = ();

    type Properties = Props;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let mut hints = self.props.hints.clone();
        hints.sort_by_key(|hint| !matches!(hint.typ,HintType::Target));
        let li = hints.iter().map(|hint| {
            html! {
                <li onclick=hint.select.reform(|_| ()) class="box is-clickable">
                    {if matches!(hint.typ,HintType::Target) { html! {<label class="label">{"ターゲット"}</label>}} else {html!{}}}
                    <a>{hint.text.as_str()}</a>
                </li>
            }
        });
        html! {
            <ul>
                {for li}
            </ul>
        }
    }
}

