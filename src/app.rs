use sycamore::prelude::*;

use crate::components::request;

#[component]
pub fn App() -> View {
    view! {
        div(class="w-screen h-screen scroll-inner") {
            request::RequestComponent()
        }
    }
}