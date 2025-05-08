use once_cell::sync::Lazy;
use reqwest::{Client, header::ACCEPT};
use yew::prelude::*;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

#[allow(non_snake_case)]
#[function_component]
fn App() -> Html {
    let dad_joke_content = Box::leak(Box::new(use_state(|| "dadjoke".to_owned())));
    let onclick = {
        let get_dad_joke = |ev: MouseEvent| {
            wasm_bindgen_futures::spawn_local(async {
                let dad_joke = CLIENT
                    .get("https://icanhazdadjoke.com/")
                    .header(ACCEPT, "text/plain")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                dad_joke_content.set(dad_joke);
            });
        };
        Callback::from(get_dad_joke)
    };

    html! {
        <div class={classes!("flex", "justify-center", "items-center", "h-screen", "bg-gradient-to-r", "from-blue-500", "to-purple-600")}>
            <div class={classes!("text-center", "p-6", "rounded-lg", "bg-white", "shadow-lg", "max-w-md", "w-full")}>
                <button {onclick} class={classes!("bg-indigo-600", "text-white", "font-semibold", "py-2", "px-6", "rounded-lg", "shadow-lg", "hover:bg-indigo-700", "transition", "duration-300", "transform", "hover:scale-105")}>
                    {"Tell a Dad Joke"}
                </button>
                <p class={classes!("mt-6", "text-xl", "text-gray-800", "font-medium", "italic")}>
                    { &*dad_joke_content.clone() }
                </p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
