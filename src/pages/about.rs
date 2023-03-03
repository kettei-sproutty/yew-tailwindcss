use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
         <div class="h-full mt-10 flex justify-center items-center">
            <h1 class="text-4xl font-bold">{"About page!"}</h1>
            <button class="btn btn-primary" {onclick}>{ "Go to Home!" }</button>
        </div>
    }
}
