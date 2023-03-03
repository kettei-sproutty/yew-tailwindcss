use yew::prelude::*;
use yew_router::BrowserRouter;

pub mod components;
pub mod pages;
pub mod router;

use components::*;
use router::*;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="min-h-screen">
                <Nav />
                <main>
                    <AppRouter />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
