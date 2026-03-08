use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    static CSS: Asset = asset!("/assets/main.css");
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DofView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div {id: "title",
            h1 { "Hot Dog 🌭"}
        }
    }
}

#[component]
fn DofView() -> Element {
    rsx! {
        div {id: "dogview",
            img { src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg", max_height: "300px" },

        }
        div { id:"buttons",
            button { id: "Skip", "Skip" }
            button { id: "Save", "Save" }
        }
    }
}
