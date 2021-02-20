#[allow(irrefutable_let_patterns)]
use web_sys::Element;
use yew::prelude::*;

pub struct Order {
    link: ComponentLink<Self>,
    props: Props,
    detail_ref: NodeRef,
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub id: i64,
}

#[derive(Clone, Debug, Copy)]
pub enum Msg {
    ToggleDetails,
}

impl Component for Order {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            detail_ref: NodeRef::default(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Msg::ToggleDetails = msg {
            if let Some(element) = self.detail_ref.cast::<Element>() {
                let class_list = element.class_list();
                class_list.toggle("is_closed");
            } else {
            }
        }
        true
    }
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
                <h2> { "Order #"} {self.props.id} </h2>
                <button
                onclick=self.link.callback(|_| Msg::ToggleDetails)>{ "Show/Hide Details" }</button>

                <div ref={self.detail_ref.clone()}>
                    <ul>
                        <li> { "Price: 10$"} </li>
                        <li> { "Wight: 4kg"} </li>
                    </ul>
                </div>
            </div>

        }
    }
}
