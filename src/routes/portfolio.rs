use yew::prelude::*;

pub struct Portfolio;

impl Component for Portfolio {
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
                <h1>{"PORTFOLIO PAGE"}</h1>
                <img src="https://cdn.dribbble.com/users/686678/screenshots/11012795/media/f71c393120e4d9f55d51e4a8507fc8ca.png" />
            </>
        }
    }
}