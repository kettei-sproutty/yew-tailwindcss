use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn route_match(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Profile => html! { <div>{"Profile"}</div> },
        Route::NotFound => html! { <div class="text-red-500">{"404"}</div> },
    }
}

#[function_component(AppRouter)]
pub fn app_router() -> Html {
    return html! ( <Switch<Route> render={route_match} /> );
}
