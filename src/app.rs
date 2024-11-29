use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


#[derive(Serialize, Deserialize)]
struct RequestArgs<'a> {
    url: &'a str,
}


// we don't need to have lifetimes because we will own this data here

// lifetimes -> data being sent as it will be gone after the command is invoked
// owned <- data recieved from command
#[derive(Serialize, Deserialize)]
struct RequestCommandArgs<'a> {
    url: &'a str,
    method: &'a str,
}

#[derive(Serialize, Deserialize)]
struct RequestResponse {
    data: String,
    status: u16,
}

#[component]
pub fn App() -> View {
    view! {
        div(class="w-screen h-screen") {
            RequestComponent()
        }
    }
}

#[component]
fn RequestComponent() -> View {
    // data signals
    let request_value = create_signal(String::new());
    let request_result = create_signal(String::new());
    let request_method = create_signal(String::from("GET"));
    let status_code = create_signal(0);

    // conditional signals
    let result_show = create_signal(true);
    
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
        div(class="flex flex-col p-2 space-y-2") {
            // request builder area
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
            div(class="")
            // response area
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
}
