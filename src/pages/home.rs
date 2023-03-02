use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::About));

    html! {
        <div class="h-full mt-10 flex justify-center items-center">
            <h1 class="text-4xl font-bold">{"Hello, Yew and Tailwind CSS!"}</h1>
            <button class="btn btn-primary" {onclick}>{ "Go to About!" }</button>
        </div>
    }
}
