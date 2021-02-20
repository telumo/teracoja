use yew::prelude::*;
use yew_router::prelude::*;

struct AppRouter {}
 
#[derive(Switch, Debug, Clone)]
enum AppRoute {
    #[to = "/order/{id}"]
    Order(i64),

    #[to = "/order_list"]
    OrderList,

    #[to = "/"]
    Index
}

impl Component for AppRouter{
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(_: Self::Message) -> ShouldRender {
        false
    }
    fn change(_props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let render_func = Router::render(|route: AppRoute| match route {

        });
        html! {
            <Router<AppRoute, ()> render=render_func>
        }
    }
}