use yew::prelude::*;

#[function_component]
pub fn Nav() -> Html {
    html! {
        <div class="navbar bg-primary text-primary-content">
            <a class="btn btn-ghost normal-case text-xl">{"Yew DaisyUI"}</a>
        </div>
    }
}
