pub mod about;
pub mod home;
pub mod portfolio;

use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/portfolio"]
    Portfolio,
    #[to = "/about"]
    About,
    #[to = "/"]
    Home,
   
}