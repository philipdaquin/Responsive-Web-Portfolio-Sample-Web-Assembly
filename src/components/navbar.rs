use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{AppRoute};
pub struct Navbar { 
    link: ComponentLink<Self>,
    is_active: bool,
}
pub enum Msg { 
    Toggle
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 
            link: _link,
            is_active: false,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg { 
            Msg::Toggle => { 
                self.is_active = !self.is_active;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let show_menu = if self.is_active {"nav__menu show-menu"} else { " nav__menu" };

        html! {
            <>
                <a href="#" class="scrolltop" id="scroll-top">
                    <i class="bx bx-chevron-up scrolltop-icon"></i>
                </a>
            
                <header class="l-header" id="header">
                    <nav class="nav bd-container">
                        <RouterAnchor<AppRoute> route=AppRoute::Home classes="nav__logo"><a>{"Philip Daquin"}</a></RouterAnchor<AppRoute>>
                        
                        <input type="checkbox" class="toggle" id="about-toggle" checked=self.is_active onclick=self.link.callback(|_| Msg::Toggle) />
                        <label class="label ms-2" for="about-toggle">
                            <div class="nav__toggle" id="nav-toggle">
                                <i class="bx bx-menu"></i>
                            </div>
                        </label>
                        <div class={show_menu} id="nav-menu">
                            <ul class="nav__list">
                                <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::Home classes="nav__link active-link"><a>{"Home"}</a></RouterAnchor<AppRoute>></li>
                                <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::About classes="nav__link"><a>{"About"}</a></RouterAnchor<AppRoute>></li>
                                <li class="nav__item"><RouterAnchor<AppRoute> route=AppRoute::Portfolio classes="nav__link"><a>{"Portfolio"}</a></RouterAnchor<AppRoute>></li>
                            </ul>
                        </div>
                    </nav>
                </header>
            </>
        }
    }
}

