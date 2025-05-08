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
        <div>
            <button {onclick}>{ "tell a dad joke" }</button>
            <p>{ *(&dad_joke_content.clone().as_str()) }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
