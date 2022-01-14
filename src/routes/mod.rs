pub mod blog;
pub mod home;

use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/blog"]
    Blog,
    #[to = "/"]
    Home,
}