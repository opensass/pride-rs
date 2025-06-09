#![doc = include_str!("../YEW.md")]

use crate::common::{Direction, FlagLookup, Size, Type};
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FlagProps {
    #[prop_or_default]
    pub r#type: Type,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub class: &'static str,

    #[prop_or_default]
    pub aria_label: String,

    #[prop_or(
        "display: flex; border-radius: 4px; overflow: hidden; transition: transform 0.2s ease, box-shadow 0.2s ease; cursor: pointer; position: relative;"
    )]
    pub style: &'static str,

    #[prop_or("flex-direction: column;")]
    pub horizontal_style: &'static str,

    #[prop_or("flex-direction: row;")]
    pub vertical_style: &'static str,

    #[prop_or("flex: 1; min-height: 4px; min-width: 4px;")]
    pub stripe_style: &'static str,

    #[prop_or("width: 24px; height: 24px;")]
    pub small_style: &'static str,

    #[prop_or("width: 48px; height: 32px;")]
    pub medium_style: &'static str,

    #[prop_or("width: 96px; height: 64px;")]
    pub large_style: &'static str,

    #[prop_or("position: relative; display: inline-block;")]
    pub container_style: &'static str,

    #[prop_or(
        "position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); background-color: #333; color: white; padding: 8px 12px; border-radius: 4px; font-size: 12px; white-space: nowrap; transition: opacity 0.2s ease, visibility 0.2s ease; z-index: 1000; pointer-events: none; opacity: 0; visibility: hidden;"
    )]
    pub tooltip_style: &'static str,

    #[prop_or("flag-container")]
    pub container_class: &'static str,

    #[prop_or("flag")]
    pub flag_class: &'static str,

    #[prop_or("stripe")]
    pub stripe_class: &'static str,

    #[prop_or("tooltip")]
    pub tooltip_class: &'static str,
}

#[function_component(Flag)]
pub fn flag(props: &FlagProps) -> Html {
    let config = props.r#type.config();

    if config.is_none() {
        log::warn!("Flag configuration not found for type: {:?}", props.r#type);
        return html! {};
    }

    let config = config.unwrap();
    let tooltip_id = format!("tooltip-{}", props.r#type.as_ref());

    let direction_style = match config.direction {
        Direction::Horizontal => props.horizontal_style,
        Direction::Vertical => props.vertical_style,
    };

    let size_style = match props.size {
        Size::Small => props.small_style,
        Size::Medium => props.medium_style,
        Size::Large => props.large_style,
    };

    let full_style = format!("{} {} {}", props.style, size_style, direction_style);
    let full_class = format!("{} {}", props.flag_class, props.class);

    let is_hovered = use_state(|| false);

    let on_mouse_over = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };

    let on_mouse_out = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };

    let on_focus = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(true))
    };

    let on_blur = {
        let is_hovered = is_hovered.clone();
        Callback::from(move |_| is_hovered.set(false))
    };

    let on_key_down = {
        Callback::from(move |e: KeyboardEvent| {
            let key = e.key();
            if key == "Enter" || key == " " {
                e.prevent_default();
                log::debug!("Selected flag: {}", config.name);
            }
        })
    };

    let tooltip_style = if *is_hovered {
        format!("{} opacity: 1; visibility: visible;", props.tooltip_style)
    } else {
        props.tooltip_style.to_string()
    };

    html! {
        <div class={props.container_class} style={props.container_style}>
            <div
                class={full_class}
                style={full_style}
                role="img"
                aria-label={props.aria_label.clone()}
                aria-describedby={tooltip_id.clone()}
                aria-roledescription="flag"
                aria-keyshortcuts="Enter Space"
                tabindex=0
                onkeydown={on_key_down}
                onmouseover={on_mouse_over.clone()}
                onmouseout={on_mouse_out.clone()}
                onfocus={on_focus}
                onblur={on_blur}
            >
                { for config.colors.iter().enumerate().map(|(i, color)| {
                    html! {
                        <div
                            key={format!("{}-{}", props.r#type.as_ref(), i)}
                            class={props.stripe_class}
                            style={format!("{} background-color: {};", props.stripe_style, color)}
                            aria-hidden="true"
                        />
                    }
                }) }
            </div>
            <div id={tooltip_id} class={props.tooltip_class} role="tooltip" style={tooltip_style}>
                { &config.name }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct FlagSectionProps {
    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub flags: Vec<Type>,

    #[prop_or_default]
    pub id: &'static str,

    #[prop_or("margin-bottom: 32px;")]
    pub section_style: &'static str,

    #[prop_or(
        "font-family: 'SF Pro Text', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 14px; font-weight: 600; color: #333; margin-bottom: 12px; padding-left: 4px;"
    )]
    pub section_title_style: &'static str,

    #[prop_or(
        "background-color: #ffffff; border: 2px dashed #7b61ff; border-radius: 8px; padding: 12px; display: flex; flex-wrap: wrap; gap: 8px; align-items: center; min-height: 48px; transition: border-color 0.2s ease;"
    )]
    pub container_style: &'static str,

    #[prop_or(
        "color: #666; font-style: italic; font-size: 12px; text-align: center; width: 100%; padding: 16px;"
    )]
    pub empty_state_style: &'static str,

    #[prop_or("section")]
    pub section_class: &'static str,

    #[prop_or("section-title")]
    pub section_title_class: &'static str,

    #[prop_or("flag-container")]
    pub container_class: &'static str,

    #[prop_or("empty-state")]
    pub empty_state_class: &'static str,
}

#[function_component(FlagSection)]
pub fn flag_section(props: &FlagSectionProps) -> Html {
    let heading_id = format!("{}-heading", props.id);
    let description_id = format!("{}-description", props.id);

    html! {
        <section
            class={props.section_class}
            style={props.section_style}
            aria-labelledby={heading_id.clone()}
            role="region"
        >
            <h2
                id={heading_id.clone()}
                class={props.section_title_class}
                style={props.section_title_style}
            >
                { &props.title }
            </h2>
            <div
                class={props.container_class}
                style={props.container_style}
                role="group"
                aria-labelledby={heading_id}
                aria-describedby={description_id.clone()}
                aria-roledescription="flag group"
            >
                if props.flags.is_empty() {
                    <div
                        id={description_id}
                        class={props.empty_state_class}
                        style={props.empty_state_style}
                        aria-live="polite"
                    >
                        { "No flags available in this category" }
                    </div>
                } else {
                    { for props.flags.iter().enumerate().map(|(i, flag_type)| {
                        html! {
                            <Flag
                                key={format!("{}-{}-{}", props.id, flag_type.as_ref(), i)}
                                r#type={*flag_type}
                                size={Size::Medium}
                                aria_label={flag_type.as_ref().to_string()}
                            />
                        }
                    }) }
                }
            </div>
        </section>
    }
}
