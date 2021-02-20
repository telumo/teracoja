use yew::prelude::*;
use yew::services::ConsoleService;

use crate::app_router::{AppRoute, Link};
use crate::components::counter::Counter;

pub struct Home {
    link: ComponentLink<Self>, // Link to component's scope for creating callbacks
    value: i64,
}

pub enum Msg {
    AddOne,
    MinusOne,
}

impl Component for Home {
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
                <Link route=AppRoute::OrderList>{"Order List"}</Link>
                <Link route=AppRoute::Order(self.value)>{"Order #"}{self.value}</Link>
            </div>
        }
    }
}
