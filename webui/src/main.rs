use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct CounterProps {
    multiplier: i64,
}

#[function_component]
fn Counter(CounterProps { multiplier }: &CounterProps) -> Html {
    let counter = use_state(|| 0);

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
    <div>
        <button class={"btn btn-primary"}{onclick}>{ "Increment value" }</button>
        <p>{*counter} {"|"} {(*counter) * multiplier}</p>
    </div>
    }
}

#[function_component]
fn MainPage() -> Html {
    html! {
    <>
            <h1>{"Crypto Trend Following"}</h1>
            <h4>{"An automated system"}</h4>
            {vec![1,2,3,4,5].into_iter().map(|i| html! {<Counter multiplier={i.clone()} />}).collect::<Html>()}
    </>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div class={"container"}>
            <MainPage/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
