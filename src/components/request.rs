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
struct RequestCommandArgs<'a> {
    url: &'a str,
    method: &'a str,
}

#[derive(Serialize, Deserialize)]
struct RequestResponse {
    data: String,
    status: u16,
}

#[derive(Clone, PartialEq)]
struct Param {
    key: String,
    val: String,
}

#[component]
pub fn RequestComponent() -> View {
    // data signals
    let request_value = create_signal(String::new());
    let request_result = create_signal(String::new());
    let request_method = create_signal(String::from("GET"));
    let status_code = create_signal(0);
    let body_content = create_signal(String::new());

    // param tab
    let params = create_signal(Vec::<Param>::new());
    let new_key = create_signal(String::new());
    let new_value = create_signal(String::new());

    let build_url = create_memo(move || {
        let base_url = request_value.get_clone();
        let params_list = params.get_clone();

        if params_list.is_empty() {
            base_url
        } else {
            let query_string = params_list
                .iter()
                .map(|param| format!("{}={}", param.key, param.val))
                .collect::<Vec<_>>()
                .join("&");

            format!("{}?{}", base_url, query_string)
        }
    });

    // conditional signals
    let result_show = create_signal(true);
    let active_tab = create_signal(0);
    
    let handle_submit = move |_| {
        result_show.set(false);
        let url = build_url.get_clone();
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

    let handle_tab = move |e: web_sys::KeyboardEvent| {
        if e.key() == "Tab" {
            e.prevent_default();
            if let Some(target) = e.target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok()) {
                    let start = target.selection_start().unwrap().unwrap();
                    let end = target.selection_end().unwrap().unwrap();
                    let value = target.value();

                    let before = &value[..start as usize];
                    let after = &value[end as usize..];
                    let new_value = format!("{}  {}", before, after);

                    target.set_value(&new_value);

                    let new_pos = start + 2;
                    target.set_selection_start(Some(new_pos)).unwrap();
                    target.set_selection_end(Some(new_pos)).unwrap();

                    body_content.set(new_value);
            }
        }
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

            div(class="text-sm text-gray-600 mt-2") {
                "Complete URL: " (build_url.get_clone())
            }
            // tabs
            div(class="w-full mx-auto p-4") {
                div(class="flex flex-col") {
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
                    }
                    div(class="my-4") {
                        (match active_tab.get() {
                            0 => view! {
                                    div(class="flex flex-col space-y-2") { 
                                        div(class="flex flex-row space-x-2") {
                                            input(
                                                "type"="text",
                                                bind:value=new_key,
                                                placeholder="Parameter key",
                                                class="border rounded p-2"
                                            )
                                            input(
                                                "type"="text",
                                                bind:value=new_value,
                                                placeholder="Parameter key",
                                                class="border rounded p-2"
                                            )
                                            button(
                                                class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow",
                                                on:click=move |_| {
                                                    let mut current = params.get_clone();
                                                    current.push(Param {
                                                        key: new_key.get_clone(),
                                                        val: new_value.get_clone()
                                                    });
                                                    params.set(current);
                                                    // Clear the inputs after adding
                                                    new_key.set(String::new());
                                                    new_value.set(String::new());
                                                }
                                            ) {
                                                "Add Parameter"
                                            }
                                        }
                                        div(class="flex flex-col space-y-2 overflow-auto max-h-32 shadow-inner") {
                                            Indexed(
                                                list=params,
                                                view=move |param| {
                                                    let param = param.clone();
                                                    let key_to_remove = param.key.clone();
                                                    view! {
                                                        div(class="flex flex-row items-center justify-between bg-gray-50 rounded-lg p-2 hover:bg-gray-100 group") {
                                                            div(class="flex items-center space-x-2") {
                                                                span(class="font-medium text-gray-600") { (param.key.clone()) }
                                                                span(class="text-gray-400") { "=" }
                                                                span(class="text-gray-800") { (param.val.clone()) }
                                                            }
                                                            button(
                                                                class="text-gray-400 hover:text-red-500 hover:bg-red-50 rounded px-2 py-1 opacity-0 group-hover:opacity-100 transition-opacity",
                                                                on:click=move |_| {
                                                                    let mut current = params.get_clone();
                                                                    current.retain(|p| p.key != key_to_remove);
                                                                    params.set(current);
                                                                }
                                                            ) {
                                                                "remove"
                                                            }
                                                        }
                                                    }
                                                }
                                            )
                                        }
                                    }
                            },
                            1 => view! {
                                div{ 
                                    textarea(
                                        bind:value=body_content,
                                        on:keydown=handle_tab,
                                        placeholder="Content here!",
                                        class="w-full bg-gray-100 h-32 p-2 border rounded font-mono text-sm"
                                    )
                                 }
                            },
                            _ => view! { div{} }
                        })
                    }
                }
            }

            // response area
            div(class="flex flex-col p-2") {
                (if result_show.get() {
                    view! {
                        div(class="border-2 shadow-sm text-xs p-2") {
                            p { "Status code: " (status_code)}
                        }
                        div(class="overflow-auto whitespace-pre-wrap font-mono text-sm bg-gray-100 p-4 rounded max-h-64") {
                            pre {
                                (request_result)   
                            }
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
