// use lol_alloc::LockedAllocator;
use once_cell::sync::Lazy;
use reqwest::{Client, header::ACCEPT};
use yew::prelude::*;
// use std::sync::{Arc, Mutex};
// use yew::{html::IntoPropValue, prelude::*};
//
// struct UseMutRef<F> {
//     init_fn: F,
// }
//
// impl<T: 'static, F: FnOnce() -> T> Hook for UseMutRef<F> {
//     type Output = Arc<Mutex<T>>;
//     fn run(self, ctx: &mut yew::HookContext) -> Self::Output {
//         Arc::new(Mutex::new(ctx.into_prop_value()))
//     }
// }
//
// pub fn use_async_hook<T: 'static, F>(init_fn: F) -> impl Hook<Output = Arc<Mutex<T>>>
// where
//     F: FnOnce() -> T,
// {
//     todo!();
// }

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

#[allow(non_snake_case)]
#[function_component]
fn App() -> Html {
    let dad_joke_content = Box::leak(Box::new(use_state(|| {
        "yes I'm testing in prod don't @ me".to_owned()
    })));
    let onclick = {
        let get_dad_joke = |_ev: MouseEvent| {
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
            <div>
                <button {onclick}>
                    {"Tell a Dad Joke"}
                </button>
                <p>
                    { &*dad_joke_content.clone() }
                </p>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
