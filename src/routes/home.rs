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
                //<!-- Current Projects -->
                <section class="project section bd-container">
                    <div class="project__container bd-grid">
                        <div class="project__data">
                            <i class="bx bx-traffic-cone project__icon" ></i>
                        </div>

                        <div>
                            <h2 class="project__title">{"Current Projects in the Workshop👷"}</h2>
                            <h3 class="project__subtitle">{"Porteurs | Automated RideSharing Services"}</h3>
                            <p class="project_description">{"A Substrate Based Project Aimed At Revolutionalising Mobility Services With Blockchain Services"}</p>
                        </div>

                        <div>
                            <a href="#" class="button button-white">{"Look at the Prototype"}
                            <i class= "bx bxl-figma"></i>  
                            </a>
                        </div>
                    </div>
                </section>
                //<!-- Portfolio -->
                <section class="portfolio section bd-container" id="portfolio ">
                    <span class="section-subtitle">{"My recent work"}</span>
                    <h2 class="section-title">{"Portfolio"}</h2>

                    <div class="portfolio__nav">
                    <span class="portfolio__item active-portfolio" data-filter="all">{"All"}</span>
                    <span class="portfolio__item" data-filter=".web">{" Web"}</span>
                    <span class="portfolio__item" data-filter=".ux">{"UI/UX"}</span>
                    <span class="portfolio__item" data-filter=".app">{"App"}</span>
                    </div>

                    <div class="portfolio__container bd-grid">
                    <div class="portfolio__content mix web">
                        <a href="#"><img src="assets/work1.jpg" alt="" class="portfolio__img"/></a>
                        <div class="portfolio__data">
                        <span class="portfolio__subtitle">{"Web Development"}</span>
                        <a href=""><h2 class="portfolio__title">{"New portfolio of work done for a client."}</h2></a>
                        <a href="#" class="button button-link">{"View Details"}</a>
                        </div>
                    </div>

                    <div class="portfolio__content mix web">
                        <a href="#"><img src="assets/work2.jpg" alt="" class="portfolio__img"/></a>
                        <div class="portfolio__data">
                        <span class="portfolio__subtitle">{"UX Design"}</span>
                        <a href=""><h2 class="portfolio__title">{"New portfolio of work done for a client."}</h2></a>
                        <a href="#" class="button button-link">{"View Details"}</a>
                        </div>
                    </div>

                    <div class="portfolio__content mix ux">
                        <a href="#"><img src="assets/work3.jpg" alt="" class="portfolio__img"/></a>
                        <div class="portfolio__data">
                        <span class="portfolio__subtitle">{"Web Development"}</span>
                        <a href=""><h2 class="portfolio__title">{"New portfolio of work done for a client."}</h2></a>
                        <a href="#" class="button button-link">{"View Details"}</a>
                        </div>
                    </div>
                    
                    <div class="portfolio__content mix ux">
                        <a href="#"><img src="assets/work4.jpg" alt="" class="portfolio__img"/></a>
                        <div class="portfolio__data">
                        <span class="portfolio__subtitle">{"Web Development"}</span>
                        <a href=""><h2 class="portfolio__title">{"New portfolio of work done for a client."}</h2></a>
                        <a href="#" class="button button-link">{"View Details"}</a>
                        </div>
                    </div>
                    
                    <div class="portfolio__content mix app">
                        <a href="#"><img src="assets/work5.jpg" alt="" class="portfolio__img"/></a>
                        <div class="portfolio__data">
                        <span class="portfolio__subtitle">{"Mobile App"}</span>
                        <a href=""><h2 class="portfolio__title">{"New portfolio of work done for a client."}</h2></a>
                        <a href="#" class="button button-link">{"View Details"}</a>
                        </div>
                    </div>
                    
                    <div class="portfolio__content mix app">
                        <a href="#"><img src="assets/work6.jpg" alt="" class="portfolio__img"/></a>
                        <div class="portfolio__data">
                        <span class="portfolio__subtitle">{"Mobile App"}</span>
                        <a href=""><h2 class="portfolio__title">{"New portfolio of work done for a client."}</h2></a>
                        <a href="#" class="button button-link">{"View Details"}</a>
                        </div>
                    </div>
                    </div>
                </section>
                //<!-- Testimonial -->
                <section class="testimonial  section bd-contianer">
                    <span class="section-subtitle">{"My client saying"}</span>
                    <h2 class="section-title">{"Testimonial"}</h2>

                    <div class="testimonial__container swiper-container">
                    <div class="swiper-wrapper">
                        <div class="testimonial__content swiper-slide">
                        <img src="assets/testimonial1.jpg" alt="" class="testimonial__img"/>
                        <h3 class="testimonial__title">{"Nik Holly"}</h3>
                        <span class="testimonial__client">{"Client"}</span>
                        <p class="testimonial__description">{"This person built the next generation of mobility services that led to revolutionise the future of cities"}</p>
                        </div>

                        <div class="testimonial__content swiper-slide">
                        <img src="assets/testimonial2.jpg" alt="" class="testimonial__img"/>
                        <h3 class="testimonial__title">{"Sara Mill"}</h3>
                        <span class="testimonial__client">{"Client"}</span>
                        <p class="testimonial__description">{"This person built the next generation of mobility services that led to revolutionise the future of cities"}</p>
                        </div>

                        <div class="testimonial__content swiper-slide">
                        <img src="assets/testimonial3.jpg" alt="" class="testimonial__img"/>
                        <h3 class="testimonial__title">{"Clay Mitchel"}</h3>
                        <span class="testimonial__client">{"Client"}</span>
                        <p class="testimonial__description">{"This person built the next generation of mobility services that led to revolutionise the future of cities"}</p>
                        </div>
                    </div>
                    <div class="swiper-pagination"></div>
                    </div>
                </section>

                //<!-- Contact Me  -->
                <section class="contact section section bd-container" id="contact">
                    <span class="section-subtitle">{"For Project"}</span>
                    <h2 class="section-title">{"Contact Me"}</h2>

                    <div class="contact__container bd-grid">

                    <div class="contact__box">
                        <i class="bx bx-home-alt contact__icon"></i>
                        <h3 class="contact__title">{"Location"}</h3>
                        <span class="contact__description">{"Cupertuino - San Francisco"}</span>
                    </div>

                    <div class="contact__box">
                        <i class="bx bx-phone contact__icon"></i>
                        <h3 class="contact__title">{"Phone"}</h3>
                        <span class="contact__description">{"999-777-999"}</span>
                    </div>

                    <div class="contact__box">
                        <i class="bx bx-envelope contact__icon"></i>
                        <h3 class="contact__title">{"Email"}</h3>
                        <span class="contact__description">{"123@gmail.com"}</span>
                    </div>

                    <div class="contact__box">
                        <i class="bx bx-chat contact__icon"></i>
                        <h3 class="contact__title">{"Chat"}</h3>
                        <div>
                        <a href="#" class="contact__social"><i class="bx bxl-linkedin"></i></a>
                        <a href="#" class="contact__social"><i class="bx bxl-github"></i></a>
                        <a href="#" class="contact__social"><i class="bx bxl-paypal"></i></a>
                        </div>
                    </div>
                    <form action="" class="contact__form"> 
                        <div class="contact__input">
                        <input type="text" placeholder="Name" class="contact__input"/>
                        <input type="email" placeholder="Email" class="contact__input"/>
                        </div>

                        <div class="contact__input">
                        <input type="text" placeholder="Project" class="contact__input"/>
                        <input type="number" placeholder="Number" class="contact__input"/>
                        </div>
                        <textarea name="" id="" cols="0" rows="10" placeholder="Message "  class="contact__input"></textarea>
                        <input type="submit" value="Send Message" class="button contact__button"/>
                    </form>
                    </div>
                </section>
                
                <script>
                    {"
                        
                    // Show menu
                    const showMenu = (toggleId, navId) => { 
                        const toggle = document.getElementById(toggleId),
                        nav = document.getElementById(navId)

                        if(toggle && nav) { 
                            toggle.addEventListener('click', () => { 
                                nav.classList.toggle('show-menu')
                            })
                        }
                    }

                    showMenu('nav-toggle', 'nav-menu')
                    //  Remove Menu Mobile 
                    const navLink = document.querySelectorAll('.nav__link')
                    function linkAction() { 
                        const navMenu = document.getElementById('nav-menu')
                        navMenu.classList.remove('show-menu')
                    }
                    navLink.forEach(n => n.addEventListener('click', linkAction))


                    // Scroll Sections Active lInk
                    const sections = document.querySelectorAll('section[id]')

                    function scrollActive() { 
                        const scrollY = window.pageYOffset

                        sections.forEach(current => { 
                            const sectionHeight = current.offsetHeight
                            const sectionTop = current.offsetTop - 50 
                            sectionId = current.getAttribute('id')

                            if(scrollY > sectionTop && scrollY <= sectionTop + sectionHeight) { 
                                document.querySelectory('.nav__menu a[href*=' + sectionId + ']').classList.add('active-link')
                            } else { 
                                document.querySelectory('.nav__menu a[href*=' + sectionId + ']').classList.remove('active-link')

                            }
                        })
                    }

                    window.addEventListener('scroll', scrollActive)

                    // Change Background Header
                    function scrollHeader() { 
                        const header = document.getElementById('header')
                        if(this.scrollY >= 200) header.classList.add('scroll-header'); else header.classList.remove('scroll-header')
                    }

                    window.addEventListener('scroll', scrollHeader)

                    // Show Scroll TOp
                    function scrollTop() { 
                        const scrollTop = document.getElementById('scroll-top')
                        if(this.scrollY >= 560) scrollTop.classList.add('show-scroll'); else scrollTop.classList.remove('show-scroll')
                    }

                    window.addEventListener('scroll', scrollTop)

                    // Mix It up Filter Portfolio
                    const mixer = mixitup('.portfolio__container', { 
                        selectors: { 
                            target: '.portfolio__content'
                        },
                        animation: { 
                            duration: 400
                        }
                    });
                    // Link active partfolio 
                    const linkPortfolio = document.querySelectorAll('.portfolio__item')

                    function activePortfolio() { 
                        if(linkPortfolio) { 
                            linkPortfolio.forEach(l => l.classList.remove('active-portfolio'))
                            this.classList.add('active-portfolio')
                        }
                    }
                    linkPortfolio.forEach(l => l.addEventListener('click', activePortfolio))


                    // Swipe Carousel
                    const swiper = new Swiper('.testimonial__container', {
                        spaceBetween: 16,
                        loop: true,
                        grabCursor: true,
                        // If we need pagination
                        pagination: {
                        el: '.swiper-pagination',
                        clickable: true, 
                        },
                        breakpoints:  { 
                            640: { 
                                slidesPerView: 2,
                            },
                            1024: { 
                                slidesPerView: 3,
                            },

                        }
                    });
                    gsap.from('.home__img', { 
                        opacity: 0, duration: 2, delay: .5, x: 60
                    })
                    gsap.from('.home__data', { 
                        opacity: 0, duration: 2, delay: .8, y: 25
                    })
                    gsap.from('.home__greeting, .home__name, home__profession, .home__button', { 
                        opacity: 0, duration: 2, delay: 1, y: 25, ease: 'expo.out', stagger: .2
                    })
                    gsap.from('.nav__logo, .nav__toggle', { 
                        opacity: 0, duration: 2, delay: 1.5, y: 25, ease: 'expo.out', stagger: .2
                    })
                    gsap.from('.nav__item', { 
                        opacity: 0, duration: 2, delay: 1.8, y: 25, ease: 'expo.out', stagger: .2
                    })
                    gsap.from('.home__social-icon', { 
                        opacity: 0, duration: 2, delay: 1.8, y: 25, ease: 'expo.out', stagger: .2
                    })

                    "}
                </script>
            </>
        }
    }
}