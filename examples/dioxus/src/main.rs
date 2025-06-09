use dioxus::prelude::*;
use dioxus_logger::tracing;
use pride_rs::dioxus::{Flag, FlagSection};
use pride_rs::{Size, Type};

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");
// const MAIN_CSS: Asset = asset!("/assets/styles.css");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

fn main() {
    dioxus_logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: TAILWIND_CSS }
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            Examples {}
        }
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        section {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 {
                class: "text-3xl font-bold mb-8 text-white",
                "Pride RS Dioxus Examples"
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 {
                        class: "text-xl font-semibold mb-4 text-gray-800",
                        "Flag Sizes"
                    }
                    Flag {
                        size: Size::Small
                    }
                    Flag {
                        size: Size::Medium
                    }
                    Flag {
                        size: Size::Large
                    }
                    pre {
                        class: "w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto",
                        r##"use dioxus::prelude::*;
use pride_rs::dioxus::Flag;
use pride_rs::Size;

#[component]
fn App() -> Element {{
    rsx! {{
        Flag {{
            size: Size::Small
        }}
        Flag {{
            size: Size::Medium
        }}
        Flag {{
            size: Size::Large
        }}
    }}
}}"##
                    }
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 {
                        class: "text-xl font-semibold mb-4 text-gray-800",
                        "Flag Types"
                    }
                    Flag {
                        r#type: Type::Rainbow
                    }
                    Flag {
                        r#type: Type::Transgender
                    }
                    Flag {
                        r#type: Type::Bisexual
                    }
                    pre {
                        class: "w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto",
                        r##"use dioxus::prelude::*;
use pride_rs::dioxus::Flag;
use pride_rs::Type;

#[component]
fn App() -> Element {{
    rsx! {{
        Flag {{
            r#type: Type::Rainbow
        }}
        Flag {{
            r#type: Type::Transgender
        }}
        Flag {{
            r#type: Type::Bisexual
        }}
    }}
}}"##
                    }
                }

                div {
                    class: "flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg",
                    h2 {
                        class: "text-xl font-semibold mb-4 text-gray-800",
                        "Flag Section"
                    }
                    FlagSection {
                        id: "flags",
                        title: "Pride Flags",
                        flags: vec![
                                Type::Rainbow,
                                Type::Transgender,
                                Type::Bisexual,
                                Type::Lesbian,
                                Type::Pansexual,
                                Type::Asexual,
                                Type::NonBinary,
                                Type::Aromantic,
                                Type::Demisexual,
                                Type::Genderfluid,
                                Type::Agender,
                                Type::Polysexual,
                                Type::Omnisexual,
                                Type::Demiromantic,
                                Type::Graysexual,
                            ]
                    }
                    pre {
                        class: "w-full text-xs bg-gray-800 text-gray-100 p-4 rounded mb-4 overflow-x-auto",
                        r##"use dioxus::prelude::*;
use pride_rs::dioxus::FlagSection;
use pride_rs::Type;

#[component]
fn App() -> Element {{
    rsx! {{
        FlagSection {{
            id: "flags",
            title: "Pride Flags",
            flags: vec![
                    Type::Rainbow,
                    Type::Transgender,
                    Type::Bisexual,
                    Type::Lesbian,
                    Type::Pansexual,
                    Type::Asexual,
                    Type::NonBinary,
                    Type::Aromantic,
                    Type::Demisexual,
                    Type::Genderfluid,
                    Type::Agender,
                    Type::Polysexual,
                    Type::Omnisexual,
                    Type::Demiromantic,
                    Type::Graysexual,
                ]
        }}
    }}
}}"##
                    }
                }
            }
        }
    }
}
