


use yew_router::prelude::*;
use yew::prelude::*;
use super::pages::landingPages::LandingPages;
use super::components::header::Header;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/:user_id")]
    User { user_id: u64 },

}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {   <LandingPages/>   },
        Route::User { user_id } => html! { <h1>{ user_id }</h1> }
    }
}

#[function_component(Main)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}