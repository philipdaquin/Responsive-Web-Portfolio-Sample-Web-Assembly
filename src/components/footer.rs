use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="footer__container bd-container">
                    <h1 class="footer__title">{"Philip Daquin"}</h1>
                    <p class="footer__description">{"I am Philip Daquin and this is my first attempt to using Rust and Web Assembly for Frontend Engineering"}</p>
                <div class="footer__social">
                    <a href="#" class="footer__link"><i class="bx bxl-linkedin"></i></a>
                    <a href="#" class="footer__link"><i class="bx bxl-github"></i></a> 
                    <a href="#" class="footer__link"><i class="bx bxl-dribbble"></i></a> 
                </div>
                    <p class="footer__copy">{"©️ 2022 Gamma Finance. All Rights Reserved"}</p>
                </div>
            </footer>
        }
    }
}