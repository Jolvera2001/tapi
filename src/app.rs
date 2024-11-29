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

// let greet = move |e: SubmitEvent| {
//     e.prevent_default();
//     spawn_local_scoped(async move {
//         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//         let args = serde_wasm_bindgen::to_value(&GreetArgs {
// 			name: &name.get_clone()
// 		})
// 		.unwrap();
//         let new_msg = invoke("greet", args).await;
//         greet_msg.set(new_msg.as_string().unwrap());
//     })
// };

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
    let request_value = create_signal(String::new());
    let request_result = create_signal(String::new());
    let request_method = create_signal(String::from("GET"));
    
    let handle_submit = move |_| {
        let url = request_value.get_clone();
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&RequestArgs{
                url: &url,
            }).unwrap();
            let res = invoke("make_request", args).await;
            request_result.set(res.as_string().unwrap());
        });
    };

    view! {
        div(class="flex flex-col p-2 space-y-2") {
            div(class=" flex flex-row space-x-2") {
                select(
                    bind:value=request_method,
                    class="border rounded p-2"
                ) {
                    option(value="GET") { "GET" }
                    option(value="GET") { "POST" }
                    option(value="GET") { "PUT" }
                    option(value="GET") { "PATCH" }
                    option(value="GET") { "DELETE" }
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
            div(class="flex flex-col") {
                div(class="overflow-auto whitespace-pre-wrap font-mono text-sm bg-gray-100 p-4 rounded max-h-64") {
                    pre {
                        (request_result)   
                    }
                }
                div(class="border-2 shadow-sm text-xs p-2") {
                    p { "Status code: "}
                    p { }
                }
            }
        }
    }
}
