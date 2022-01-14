use crate::routes::{home::Home, about::About, portfolio::Portfolio, AppRoute};
use crate::components::{navbar::Navbar};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Navbar/>
                <div class="main">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Home => html!{ <Home/> },
                                AppRoute::About => html!{ <About/> },
                                AppRoute::Portfolio => html!{ <Portfolio/> },
                            }
                        })
                    />
                </div>
            </>
        }
    }
}
