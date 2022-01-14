mod app;
mod routes;
mod components;


use crate::app::Main;
fn main() {
    yew::start_app::<Main>();
}