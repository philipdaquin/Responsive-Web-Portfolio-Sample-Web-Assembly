use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{AppRoute, 
    home::Home, about::About, portfolio::Portfolio};
pub struct Navbar;

impl Component for Navbar {
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
        
                <header class="l-header" id="header">
                    <nav class="nav bd-container">
                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav__logo"><a>{"Philip Daquin"}</a></RouterAnchor<AppRoute>>
                    
                        <div class="nav__menu" id="nav-menu">
                            <ul class="nav__list">
                                <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::Home classes="nav__link"><a>{"Home"}</a></RouterAnchor<AppRoute>></li>
                                <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::About classes="nav__link"><a>{"About"}</a></RouterAnchor<AppRoute>></li>
                                <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::Portfolio classes="nav__link"><a>{"Portfolio"}</a></RouterAnchor<AppRoute>></li>
                            </ul>
                        </div>
                        <div class="nav__toggle" id="nav-toggle">
                            <i class="bx bx-menu"></i>
                        </div>
                    </nav>
                </header>
     
        }
    }
}