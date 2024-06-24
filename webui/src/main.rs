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

#[function_component(Header)]
fn header() -> Html {
    html! {
    <header class="bg-blue-700 text-white sticky top-0 z-10">
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
    <main class="max-4-wxl mx-auto grid place-content-center">
        <div class="w-96 h-8 bg-gray-100 border border-zinc-400 rounded-[15px] grid place-content-center text-slate-900 hover:bg-gray-200 hover:shadow-lg hover:shadow-inner">
            {"ASDF"}
        </div>
        <FunLogo />
    </main>
    }
}
#[function_component(FunLogo)]
fn fun_logo() -> Html {
    html! {
    <div class="min-h-screen grid place-content-center">
        <div class="bg-emerald-500 w-52 h-52 rounded-full shadow-2xl grid place-content-center">
            <div class="bg-teal-200 w-32 h-32 rounded-full shadow-2xl grid place-content-center">
                <div class="bg-red-500 hover:bg-blue-500 hover:blur-lg w-16 h-16 rounded-full shadow-2xl">
                </div>
            </div>
        </div>
    </div>
    }
}
#[function_component]
fn MainPage() -> Html {
    html! {
    <body class="min-h-screen bg-slate-50 dark:bg-zinc-100 dark:text-white">
            <Header/>
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
