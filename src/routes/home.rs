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
            <>
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
            
                <section class="about section bd-container" id="about">
                    <span class="section-subtitle">{"My History"}</span>
                    <h2 class="section-title">{"About Me"}</h2>
            
                    <div class="about__container bd-grid">
                        <div class="about__data bd-grid">
                            <p class="about__description"><span class="span-1">{"Hello, I am... 🤔"}<br/> </span>
                                {"A Product Designer and Engineer for "}<span class="span-2">{"Porteurs"}</span>{". I love designing new technologies and models that would revolutionise the Future of Housing and Future of Finance. "}
                            </p>
                            <div>
                                <span class="about__number">{"5+"}</span>
                                <span class="about__achievements">{"Years Off Experience"}</span>
                            </div>
                            
                            <div>
                                <span class="about__number">{"29"}</span>
                                <span class="about__achievements">{"Completed Projects"}</span>
                            </div>
                            
                            <div>
                                <span class="about__number">{"07"}</span>
                                <span class="about__achievements">{"Best Work Awards"}</span>
                            </div>
                        </div>
                        <img src="https://github.com/bedimcode/responsive-portfolio-Clay-Doe/blob/main/assets/img/about.jpg?raw=true" alt="" class="about__img"/>
                    </div>
                </section>
                <section class="qualification section bd-container" id="about">
                    <span class="section-subtitle">{"Experience and Education"}</span>
                    <h2 class="section-title">{"Qualification"}</h2>
                    <div class="qualification__container bd-grid">
                        <div class="qualification__content">
                        <h2 class="qualification__title">
                            <i class="bx bx-briefcase-alt qualification__title-icon"></i>
                            {"Work Experience"}
                        </h2>
                        //<!-- Examples -->
                        <div class="bd-grid">
                            <div class="qualification__data">
                            <h3 class="qualification__area">{"Runtime Engineer"}</h3>
                            //<!-- Rust Engineer -->
                            <div class="qualification__box">
                                <span class="qualification__work">
                                    <i class="bx bx-briefcase-alt qualification__icon"></i>
                                    {"Rust Protocol Engineer"}
                                    </span>
                                <span class="qualification__work">
                                    <i class="bx bx-calendar-alt qualification__icon"></i>
                                    {"January 2020 - Aug 2024"}
                                </span>
                            </div>
                            <div class="qualification__box">
                                <span class="qualification__work">
                                <i class="bx bx-briefcase-alt qualification__icon"></i>
                                {"Substrate Engineer"}
                                </span>
                                <span class="qualification__work">
                                <i class="bx bx-calendar-alt qualification__icon"></i>
                                {"January 2020 - Aug 2024"}
                                </span>
                            </div>
                            <div class="qualification__box">
                                <span class="qualification__work">
                                <i class="bx bx-briefcase-alt qualification__icon"></i>
                               {" Yew Engineer"}
                                </span>
                                <span class="qualification__work">
                                <i class="bx bx-calendar-alt qualification__icon"></i>
                                {"January 2020 - Aug 2024"}
                                </span>
                            </div>
                            <div class="qualification__box">
                                <span class="qualification__work">
                                <i class="bx bx-briefcase-alt qualification__icon"></i>
                                {"Juniper/ GraphQl Engineer"}
                                </span>
                                <span class="qualification__work">
                                <i class="bx bx-calendar-alt qualification__icon"></i>
                                {"January 2020 - Aug 2024"}
                                </span>
                            </div>
                            </div>
                            //<!-- Product Design  -->
                            <div class="bd-grid">
                            <div class="qualification__data">
                                <h3 class="qualification__area">{"Product Designer"}</h3>
                                <div class="qualification__box">
                                <span class="qualification__work">
                                    <i class="bx bx-briefcase-alt qualification__icon"></i>
                                    {"Figma, Adobe XD, FramerX"}
                                </span>
                                <span class="qualification__work">
                                    <i class="bx bx-calendar-alt qualification__icon"></i>
                                    {"January 2020 - Dec 2024"}
                                </span>
                                </div>
                            </div>              
                            </div>
                        </div>
                        //<!-- Education -->
                        <div class="qualification__container bd-grid">
                            <div class="qualification__content">
                                <h2 class="qualification__title">
                                    <i class="bx bx-book qualification__title-icon"></i>
                                {"Education"}
                                </h2>
                                <div class="bd-grid">
                                    <div class="qualification__data">
                                    <h3 class="qualification__area">{"Computer Technician"}</h3>
                                    //<!-- Rust Engineer -->
                                    <div class="qualification__box">
                                        <span class="qualification__work">
                                            <i class="bx bx-briefcase-alt qualification__icon"></i>
                                            {"Apple I"}
                                        </span>
                                        <span class="qualification__work">
                                            <i class="bx bx-calendar-alt qualification__icon"></i>
                                            {"January 1986 - Aug 2012"}
                                        </span>
                                    </div>
                                </div>
                                <div class="bd-grid">
                                    <div class="qualification__data">
                                        <h3 class="qualification__area">{"Runtime Development Engineer"}</h3>
                                       // <!-- Rust Engineer -->
                                        <div class="qualification__box">
                                        
                                        <span class="qualification__work">
                                            <i class="bx bx-calendar-alt qualification__icon"></i>
                                            {"September 2021 - December 2021"}
                                        </span>
                                        </div>
                                    </div>
                                    <div class="bd-grid">
                                        <div class="qualification__data">
                                        <h3 class="qualification__area">{"UI/ UX Google Certificate"}</h3>
                                        //<!-- Rust Engineer -->
                                        <div class="qualification__box">
                                            <span class="qualification__work">
                                            <i class="bx bx-briefcase-alt qualification__icon"></i>
                                            {"Link to Adobe Portfolio here.."}
                                            </span>
                                            <span class="qualification__work">
                                            <i class="bx bx-calendar-alt qualification__icon"></i>
                                           {" May 2021 - Nov 2021"}
                                            </span>
                                      </div>
                                    </div>
                                 </div>
                               </div>
                             </div>
                           </div>
                         </div>
                       </div>
                     </div>
                </section>

                // Services
                <section class="services bd-container" id="services">
                <span class="section-subtitle">{"What I offer "}</span>
                <h2 class="section-title">{"Services"}</h2>

                <div class="services__container bd-grid">
                    <div class="services__data">
                    <i class="bx bx-palette services__icon"></i>
                    <h3 class="services__title">{"UI/ UX Design"}</h3>
                    <p class="services__description">{" Service that I offer and work, 
                        with years of experience in the work area."}
                    </p>
                    <a href="#" class="button">{"Know More"}</a>
                    </div>

                    <div class="services__data">
                    <i class="bx bx-laptop services__icon"></i>
                    <h3 class="services__title">{"Product Design"}</h3>
                    <p class="services__description"> {"Service that I offer and work, 
                        with years of experience in the work area."}
                    </p>
                    <a href="#" class="button">{"Know More"}</a>
                    </div>

                    <div class="services__data">
                    <i class="bx bx-palette services__icon"></i>
                    <h3 class="services__title">{"Web Engineering"}</h3>
                    <p class="services__description"> {"Service that I offer and work, 
                        with years of experience in the work area."}
                    </p>
                    <a href="#" class="button">{"Know More"}</a>
                    </div>
                
                </div>
                </section>
            </>
        }
    }
}