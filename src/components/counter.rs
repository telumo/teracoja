use yew::prelude::*;

pub struct Counter {
    props: Props,
}

#[derive(Clone, Debug, Properties)]
pub struct Props{
    pub children: Children
}


impl Component for Counter {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <p class="text-purple-600 m-4">{ "Count: "}{ self.props.children.clone() }</p>
            </div>
        }
    }
}

