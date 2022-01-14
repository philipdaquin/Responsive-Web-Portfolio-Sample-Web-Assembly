use yew::prelude::*;

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
            <>
                <header class="navbar">
                    <ul>
                        <li>
                            <RouterAnchor<AppRoute> route=AppRoute::Home><a>{"Home"}</a></RouterAnchor<AppRoute>>
                        </li>
                        <li>
                            <RouterAnchor<AppRoute> route=AppRoute::Blog><a>{"Blog"}</a></RouterAnchor<AppRoute>>
                        </li>
                        <li>
                            <RouterAnchor<AppRoute> route=AppRoute::Portfolio><a>{"Portfolio"}</a></RouterAnchor<AppRoute>>
                        </li>
                    </ul>
                </header>
            </>
        }
    }
}