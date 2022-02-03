use yew::prelude::*;

struct Counter {
    value: i64,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Counter { value: 0 });

    let increment = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Counter {
                value: state.value + 1,
            })
        })
    };

    let decrement = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Counter {
                value: state.value - 1,
            })
        })
    };

    html! {
    <div class="container">
      <div class="view-box">
        <div class="view-area">
          <p class="content">{state.value}</p>
        </div>
        <div class="control-area">
          <button onclick={increment} class="cybr-btn">
            {"+1"}
            <span class="cybr-btn__glitch">{"+1"}</span>
            <span class="cybr-btn__tag">{"R25"}</span>
          </button>
          <button onclick={decrement} class="cybr-btn">
            {"-1"}
            <span class="cybr-btn__glitch">{"-1"}</span>
            <span class="cybr-btn__tag">{"R25"}</span>
          </button>
        </div>
      </div>
    </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
