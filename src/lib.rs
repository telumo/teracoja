#![recursion_limit = "640"] //https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute
mod app_router;
mod components;
mod pages;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use app_router::AppRouter;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <AppRouter/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
// self, &self, and Self
// self is the current object
// &self is a reference to the current object
// Self refers to the type of the current object
