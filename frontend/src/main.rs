use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::{html, prelude::function_component, use_effect_with_deps, use_state_eq, UseStateHandle};

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Hello, World!"}</h2>
        </div>
    }
}
