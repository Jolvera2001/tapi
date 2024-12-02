#[component]
fn BodyComponent() -> view {
    view! {
        div(class="flex border-b") {
            button(
                class=format!("px-2 py-1 {} {}",
                    "focus:outline-none",
                    if active_tab.get() == 0 { 
                        "border-b-2 border-blue-500 font-medium" 
                    } else { 
                        "text-gray-500 hover:text-blue-500" 
                    }
                ),
                on:click=move |_| active_tab.set(0)
            ) {
                "Params"
            }
            button(
                class=format!("px-2 py-1 {} {}",
                    "focus:outline-none",
                    if active_tab.get() == 1 {
                        "border-b-2 border-blue-500 font-medium"
                    } else { 
                        "text-gray-500 hover:text-blue-500" 
                    }
                ),
                on:click=move |_| active_tab.set(1)
            ) {
                "Body"
            }
            div(class="m-4") {
                (match active_tab.get() {
                    0 => view! {
                        div { "Params view!" }
                    },
                    1 => view! {
                        div{ "Body view!" }
                    },
                    _ => view! { div{} }
                })
            }
        }
    }
}