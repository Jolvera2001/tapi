use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());

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

    view! {
        div(class="flex items-center justify-center min-h-screen bg-gray-100") {
            div(class="p-6 bg-white rounded-lg shadow-lg") {
                h1(class="text-2xl font-bold text-gray-800") { "Hello Tailwind!"}
            }
        }
    }
}
