use gloo_net::http::Request;
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

#[function_component(Table)]
fn table() -> Html {
    let perps = use_state(|| vec![]);

    {
        let perps = perps.clone();
        use_effect_with((), move |_| {
            let perps = perps.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_perps: Vec<String> = Request::get("http://127.0.0.1:8080/")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                perps.set(fetched_perps);
            });
            || ()
        });
    }

    html! {
    <div>
        <p class="text-white text-sm">{perps.len()}</p>
        <div class="h-96 overflow-auto">
    {
        perps.iter().map(|perp| {
        html! {
            <div class="text-white text-xl">{perp}</div>
        }
            }).collect::<Html>()
    }
        </div>
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
    <header class="border-solid border border-t-0 border-violet-950 text-white sticky top-0 z-10 w-1/2 rounded-b-lg bg-opacity-0 bg-gradient-to-b from-indigo-900">
        <section class="max-w-4xl mx-auto p-4 flex justify-between items-center">
            <h1 class="text-3xl font-medium">{"💸 Trendfollowing"}</h1>
            <div>
                <button id="mobile-open-button" class="text-3xl sm:hidden focus:outline-none">
                {"☰"}
                </button>
                <nav class="hidden sm:block space-x-8 text-xl">
                    <a href="#trends" class="hover:opacity-80">{"Trends"}</a>
                    <a href="#trends" class="hover:opacity-80">{"Instruments"}</a>
                    <a href="#trends" class="hover:opacity-80">{"Positions"}</a>
                </nav>
            </div>
        </section>
    </header>
    }
}

#[function_component(Main)]
fn main() -> Html {
    html! {
    <main class="max-4-wxl mx-auto pl-32 pt-32">
        <div class="grid grid-cols-2">
        <div>
            <h1 class="text-8xl text-white font-black">{"The safest way"}</h1>
            <h1 class="text-8xl text-white font-light">{"to invest in crypto"}</h1>
        </div>
        <div class="grid w-full grid place-content-center">
            <span class="text-white font-black text-9xl">{"🚀"}</span>
        </div>
        </div>
    <Table />
    </main>
    }
}
#[function_component(FunLogo)]
fn fun_logo() -> Html {
    html! {
        <div class="bg-emerald-500 w-52 h-52 rounded-full shadow-2xl grid place-content-center">
            <div class="bg-teal-200 w-32 h-32 rounded-full shadow-2xl grid place-content-center">
                <div class="bg-red-500 hover:blur-lg w-16 h-16 rounded-full shadow-2xl">
                </div>
            </div>
        </div>

    }
}
#[function_component]
fn MainPage() -> Html {
    html! {
    <body class="min-h-screen absolute inset-0 -z-10 h-full w-full  [background:radial-gradient(125%_125%_at_50%_10%,#000_40%,#63e_100%)]">
        <div class="min-w-screen full flex justify-center">
            <Header />
        </div>
    <Main />
    </body>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <MainPage/>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
