#[component]
pub fn ResponseComponent() -> View {
    view! {
        div(class="flex flex-col") {
            (if result_show.get() {
                view! {
                    div(class="overflow-auto whitespace-pre-wrap font-mono text-sm bg-gray-100 p-4 rounded max-h-64") {
                        pre {
                            (request_result)   
                        }
                    }
                    div(class="border-2 shadow-sm text-xs p-2") {
                        p { "Status code: " (status_code)}
                    }
                }
            } else {
                view! {
                    p { "Loading..." }
                }
            })
        }
    }
}