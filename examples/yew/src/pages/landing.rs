use pride_rs::yew::{FlagSection, Flag};
use pride_rs::{Size, Type};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Pride RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Flag Sizes" }</h2>
                    <Flag size={Size::Small} />
                    <Flag size={Size::Medium} />
                    <Flag size={Size::Large} />
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                    { r#"use yew::prelude::*;
use pride_rs::yew::Flag;
use pride_rs::Size;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Flag size={Size::Small} />
            <Flag size={Size::Medium} />
            <Flag size={Size::Large} />
        </>
    }
}"# }
                    </pre>
                </div>
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Flag Types" }</h2>
                    <Flag r#type={Type::Rainbow} />
                    <Flag r#type={Type::Transgender} />
                    <Flag r#type={Type::Bisexual} />
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                    { r#"use yew::prelude::*;
use pride_rs::yew::Flag;
use pride_rs::Type;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Flag r#type={Type::Rainbow} />
            <Flag r#type={Type::Transgender} />
            <Flag r#type={Type::Bisexual} />
        </>
    }
}"# }
                    </pre>
                </div>
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Flag Section" }</h2>
                    <FlagSection
                        id="flags"
                        title="Pride Flags"
                        flags={vec![
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
                            ]}
                    />
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                    { r#"use yew::prelude::*;
use pride_rs::yew::FlagSection;
use pride_rs::Type;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <FlagSection
            id="flags"
            title="Pride Flags"
            flags={vec![
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
            ]}
        />
    }
}"# }
                    </pre>
                </div>
            </div>
        </div>
    }
}
