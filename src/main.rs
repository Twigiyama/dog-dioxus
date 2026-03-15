use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

//Create wrapper type
#[derive(Clone)]
struct TitleState(String);

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}



fn App() -> Element {
    use_context_provider(|| TitleState("Holt Dog 🌭".to_string()));
    static CSS: Asset = asset!("/assets/main.css");
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div {id: "title",
            h1 { "{title.0}"}
        }
    }
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(||  async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx! {

        div {id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() },

        }
        div { id:"buttons",
            button { onclick: move |_| img_src.restart(), id: "Skip", "Skip" }
            button { onclick: move |_| img_src.restart(), id: "Save", "Save" }
        }
    }
}
