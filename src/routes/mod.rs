pub mod blog;
pub mod home;
pub mod portfolio;

use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/blog"]
    Blog,
    #[to = "/"]
    Home,
    #[to = "/portfolio"]
    Portfolio,
}