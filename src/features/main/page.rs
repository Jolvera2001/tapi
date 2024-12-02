use sycamore::prelude::create_signal;
use super::components::{builder::BuilderComponent, response::ResponseComponent};

#[component]
pub fn MainPage() -> View {
    let request_value = create_signal(String::new());
    let request_result = create_signal(String::new());
    let request_method = create_signal(String::from("GET"));
    let status_code = create_signal(0);

    // conditional signals
    let result_show = create_signal(true);
    let active_tab = create_signal(0);

    view! {
        div(class="flex flex-row space-x-2") {
            BuilderComponent(
                request_method=request_method, 
                request_value=request_value, 
                request_result=request_result
                status_code=status_code,
                result_show=result_show
            )
            // body for different requests func
            // response func
        }
    }
} 