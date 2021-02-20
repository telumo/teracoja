#![recursion_limit = "640"] //https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;

mod components;
use components::counter::Counter;

struct Model {
    link: ComponentLink<Self>, // Link to component's scope for creating callbacks
    value: i64,
}

enum Msg {
    AddOne,
    MinusOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                ConsoleService::log("Added.");
            }
            Msg::MinusOne => {
                self.value -= 1;
                ConsoleService::log("Added.");
            }
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div class="flex items-center justify-center h-screen">
                <Counter>
                    { self.value }
                </Counter>
                <div class="">
                    <button class="bg-transparent hover:bg-blue-500 text-blue-700 font-semibold
                        hover:text-white py-2 px-4 border border-blue-500 hover:border-transparent rounded"
                        onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <button class="bg-transparent hover:bg-blue-700 text-blue-900 font-semibold
                        hover:text-white py-2 px-4 border border-blue-700 hover:border-transparent rounded"
                        onclick=self.link.batch_callback(|_| vec![Msg::AddOne, Msg::AddOne])>{ "+2" }</button>
                        <button class="bg-transparent hover:bg-blue-700 text-blue-900 font-semibold
                        hover:text-white py-2 px-4 border border-blue-700 hover:border-transparent rounded"
                        onclick=self.link.callback(|_| Msg::MinusOne)>{ "-1" }</button>
                </div>
            </div>
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
