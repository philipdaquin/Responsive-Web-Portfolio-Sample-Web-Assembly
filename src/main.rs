mod app;
mod routes;

use crate::app::Main;
fn main() {
    yew::start_app::<Main>();
}