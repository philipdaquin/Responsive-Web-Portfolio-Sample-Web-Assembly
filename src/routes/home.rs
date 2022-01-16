use yew::prelude::*;

pub struct Home;

impl Component for Home {
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
            
            <section class="home" id="home">
                <div class="home__container bd-container bd-grid">
                    <div class="home__data">
                        <span class="home__greeting">{" Hello👋, my name is "}</span>
                        <h1 class="home__name">{"Philip Daquin"}</h1>
                        <span class="home__profession">{" Rust Engineer "}</span>
                        <a download="" href="assets/download.png" class="button button-light">{"Download Cv"}</a>
                    </div>
                    <div class="home__social">
                        <a href="#" class="home__social-icon"> <i class= "bx bxl-facebook-square"></i></a>
                        <a href="#" class="home__social-icon"> <i class= "bx bxl-instagram"></i></a>
                        <a href="#" class="home__social-icon"> <i class= "bx bxl-twitter"></i></a>
                    </div>
                    <div class="home__img">
                        <img src="https://github.com/bedimcode/responsive-portfolio-Clay-Doe/blob/main/assets/img/home.jpg?raw=true" alt=""/>
                    </div>
                </div>
            </section>
          
           
        }
    }
}