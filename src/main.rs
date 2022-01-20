mod app;
mod routes;
mod components;
//mod interop;
mod index;

use crate::app::Main;

use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}