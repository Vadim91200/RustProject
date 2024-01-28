use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use rand::Rng;

struct Model {
    input_text: String,
    duration: u64,
    result_text: f64,
}


#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        input_text: String::default(),
        duration: 0,
        result_text: 0.0,
    });


    let on_input = {
        let state = state.clone();
        Callback::from(move |e: Event| {
            let target: EventTarget = e.target().expect("Event should have a target when dispatched");
            let input = target.unchecked_into::<HtmlInputElement>().value();
            state.set(Model {
                input_text: input,
                duration:state.duration.clone(),
                result_text: state.result_text.clone()
            });
            if state.duration!=0 {
                let falseR= FalseResult();
                state.set(Model {
                input_text: state.input_text.clone(),
                duration:state.duration.clone(),
                result_text: falseR
            });

            }
        })
    };

    let onclick1 = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                input_text: state.input_text.clone(),
                duration: 1,
                result_text: state.result_text.clone(),
            });
            if state.input_text!="" {
                let falseR= FalseResult();
                state.set(Model {
                input_text: state.input_text.clone(),
                duration:state.duration.clone(),
                result_text: falseR
            });
            }
        })
    };

    let onclick2 = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                input_text: state.input_text.clone(),
                duration: 2,
                result_text: state.result_text.clone(),
            });
            if state.input_text!="" {
                let falseR= FalseResult();
                state.set(Model {
                input_text: state.input_text.clone(),
                duration:state.duration.clone(),
                result_text: falseR
            });
            }
        })
    };

    let onclick3 = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                input_text: state.input_text.clone(),
                duration: 3,
                result_text: state.result_text.clone(),
            });
            if state.input_text!="" {
                let falseR= FalseResult();
                state.set(Model {
                input_text: state.input_text.clone(),
                duration:state.duration.clone(),
                result_text: falseR
            });
            }
        })
    };


    html! {
        <>
        <div class="card">
            <h1> {"Price Prediction"}</h1>
            <h4 for="Chosen Ticker">
                { "Enter the ticker of the company you want to look up: " }
                <input onchange={on_input}
                    id="chosen-ticker"
                    type="text"
                    value={state.input_text.clone()}
                />
            </h4>
            <div>
            <h4>{"For which duration would you like to know the expected price: "}
            <button type="button" class="time-button" onclick={onclick1}>{"30 days"}</button>
            <button type="button" class="time-button" onclick={onclick2}>{"90 days"}</button>
            <button type="button" class="time-button" onclick={onclick3}>{"365 days"}</button>
            </h4>
            </div>

            <h4>{"The expected price is: "}
                {state.result_text}
            </h4>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn FalseResult() -> f64 {
    let mut rng = rand::thread_rng();
    let n: f64 = rng.gen_range(0.0..1000.0);
    let rounded_result = (n * 1e4).round() / 1e4;

    return rounded_result;
}
