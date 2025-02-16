use dioxus::prelude::*;

mod components;
use components::product_overview;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}
        Hello { message: "World" }
        product_overview::product_overview {
            name: "Dioxus".to_string(),
            price: "$0.00".to_string(),
            rating: 5.0,
            review_count: 0,
            images: vec!["/assets/dioxus.png".to_string()],
            colors: vec!["#000000".to_string()],
            sizes: vec!["S".to_string(), "M".to_string(), "L".to_string()],
            description: "The best way to build web apps".to_string(),
            features: vec![
                "Fast".to_string(),
                "Simple".to_string(),
                "Modern".to_string(),
                "Powerful".to_string(),
            ],
        }

    }
}

#[component]
pub fn Hello(message: String) -> Element {
    let message = use_resource(get_server_data);

    rsx! {
        div { "Hello, {message.value():?}" }
    }
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    // Access a database
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok("Hello from the server!".to_string())
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
