use sycamore::{futures::spawn_local_scoped, prelude::Signal, web::View};

#[component]
pub fn BuilderComponent<'a>(
    request_method: &'a Signal<String>, 
    request_value: &'a Signal<String>, 
    request_result: &'a Signal<String>,
    status_code: &'a Signal<u16>,
    result_show: &'a Signal<bool>,
) -> View {
    
    let handle_submit = move |_| {
        result_show.set(false);
        let url = request_value.get_clone();
        let method = request_method.get_clone();
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&RequestCommandArgs{
                url: &url,
                method: &method
            }).unwrap();
            let res = invoke("make_request", args).await;
            let response: RequestResponse = serde_wasm_bindgen::from_value(res).unwrap();
            match serde_json::from_str::<serde_json::Value>(&response.data) {
                Ok(json) => request_result.set(serde_json::to_string_pretty(&json).unwrap()),
                Err(e) => request_result.set(e.to_string())
            }
            status_code.set(response.status);
        });
        result_show.set(true);
    };
    
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