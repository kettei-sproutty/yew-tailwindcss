use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="h-full mt-10 flex justify-center items-center">
            <h1 class="text-4xl font-bold">{"Hello, Yew and Tailwind CSS!"}</h1>
        </div>
    }
}
