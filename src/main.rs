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
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let skip = move |evt| {};
    let save = move |_| {
        img_src.set("https://upload.wikimedia.org/wikipedia/commons/1/13/Un_chien_Spitz_allemand.jpg")
    };
    rsx! {

        div {id: "dogview",
            img { src: "{img_src}", max_height: "300px" },

        }
        div { id:"buttons",
            button { onclick: skip, id: "Skip", "Skip" }
            button { onclick: save, id: "Save", "Save" }
        }
    }
}
