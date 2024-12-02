#[component]
fn BuilderComponent() -> view {
    view! {
        div(class=" flex flex-row space-x-2") {
            select(
                bind:value=request_method,
                class="border rounded p-2"
            ) {
                option(value="GET") { "GET" }
                option(value="POST") { "POST" }
                option(value="PUT") { "PUT" }
                option(value="PATCH") { "PATCH" }
                option(value="DELETE") { "DELETE" }
            }
            input(
                "type"="text",
                bind:value=request_value,
                placeholder="URL",
                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            )
            button(
                on:click=handle_submit,
                class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
            ) {
                "Send"
            }
        }
    }
}