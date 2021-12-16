use yew::{Component, Properties, html};

pub struct Hand {
    props: Props
}

#[derive(Properties,Clone)]
pub struct Props {
    pub hints: HandHints
}

pub type HandHints = Vec<(String,HintType)>;

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
        html! {
            format!("{:?}",self.props.hints)
        }
    }
}

