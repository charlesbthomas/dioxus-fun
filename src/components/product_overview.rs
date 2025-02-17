use dioxus::prelude::*;

#[derive(Clone, Props, PartialEq)]
pub struct ProductOverviewProps {
    name: String,
    price: String,
    rating: f32,
    review_count: u32,
    images: Vec<String>,
    colors: Vec<String>,
    sizes: Vec<String>,
    description: String,
    features: Vec<String>,
}

#[component]
pub fn ProductOverview(props: ProductOverviewProps) -> Element {
    rsx! {
        div { class: "bg-white",
            div { class: "pb-16 pt-6 sm:pb-24",
                h1 { class: "text-xl font-medium text-gray-900", "{props.name}" }
                p { class: "text-xl font-medium text-gray-900", "${props.price}" }
                div { class: "mt-4",
                    p { class: "text-sm text-gray-700", "{props.rating} out of 5 stars" }
                    p { class: "text-sm text-gray-700", "{props.review_count} reviews" }
                }
                div { class: "grid grid-cols-1 lg:grid-cols-2 lg:grid-rows-3 lg:gap-8",
                    for image in &props.images {
                        img { src: "{image}", class: "rounded-lg" }
                    }
                }
                div { class: "mt-8",
                    h2 { class: "text-sm font-medium text-gray-900", "Colors" }
                    div { class: "flex items-center gap-x-3",
                        for color in &props.colors {
                            span { class: "size-8 rounded-full border border-black/10", style: "background-color:{color}" }
                        }
                    }
                }
                div { class: "mt-8",
                    h2 { class: "text-sm font-medium text-gray-900", "Sizes" }
                    div { class: "grid grid-cols-3 gap-3 sm:grid-cols-6",
                        for size in &props.sizes {
                            span { class: "border px-3 py-3 text-sm font-medium uppercase", "{size}" }
                        }
                    }
                }
                button { class: "mt-8 flex w-full items-center justify-center rounded-md bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700", "Add to cart" }
                div { class: "mt-10",
                    h2 { class: "text-sm font-medium text-gray-900", "Description" }
                    p { class: "text-sm text-gray-500", "{props.description}" }
                }
                div { class: "mt-8 border-t border-gray-200 pt-8",
                    h2 { class: "text-sm font-medium text-gray-900", "Features" }
                    ul { class: "list-disc pl-5 text-sm text-gray-500",
                        for feature in &props.features {
                            li { "{feature}" }
                        }
                    }
                }
            }
        }
    }
}
