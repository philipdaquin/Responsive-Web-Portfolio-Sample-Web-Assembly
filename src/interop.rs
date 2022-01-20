use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsValue};

use yew_interop::declare_resources;

declare_resources!(
    gsap
    "/assets/gsap.min.js"
    
    mixitup
    "/assets/mixitup.min.js"

    swiper
    "https://unpkg.com/swiper@7/swiper-bundle.min.css"
    "https://unpkg.com/swiper@7/swiper-bundle.min.js"
    
    main
    "main.js"
);

// Swiper
#[wasm_bindgen]
extern "C" { 
    #[wasm_bindgen(js_name = Swiper)]
    pub type Swiper;
    

}