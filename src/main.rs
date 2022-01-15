mod app;
mod routes;
mod components;
mod external;

use crate::app::Main;
fn main() {
    yew::start_app::<Main>();
}