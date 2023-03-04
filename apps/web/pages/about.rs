use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
         <div class="h-full mt-10 flex justify-center items-center">
            <h1 class="text-4xl font-bold">{"About page!"}</h1>
        </div>
    }
}
