#![doc = include_str!("../DIOXUS.md")]

use crate::common::{Direction, FlagLookup, Size, Type};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Props, PartialEq, Clone)]
pub struct FlagProps {
    #[props(default)]
    pub r#type: Type,
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub class: &'static str,
    #[props(default)]
    pub aria_label: String,
    #[props(
        default = "display: flex; border-radius: 4px; overflow: hidden; transition: transform 0.2s ease, box-shadow 0.2s ease; cursor: pointer; position: relative;"
    )]
    pub style: &'static str,
    #[props(default = "flex-direction: column;")]
    pub horizontal_style: &'static str,
    #[props(default = "flex-direction: row;")]
    pub vertical_style: &'static str,
    #[props(default = "flex: 1; min-height: 4px; min-width: 4px;")]
    pub stripe_style: &'static str,
    #[props(default = "width: 24px; height: 24px;")]
    pub small_style: &'static str,
    #[props(default = "width: 48px; height: 32px;")]
    pub medium_style: &'static str,
    #[props(default = "width: 96px; height: 64px;")]
    pub large_style: &'static str,
    #[props(default = "position: relative; display: inline-block;")]
    pub container_style: &'static str,
    #[props(
        default = "position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); background-color: #333; color: white; padding: 8px 12px; border-radius: 4px; font-size: 12px; white-space: nowrap; transition: opacity 0.2s ease, visibility 0.2s ease; z-index: 1000; pointer-events: none; opacity: 0; visibility: hidden;"
    )]
    pub tooltip_style: &'static str,
    #[props(default = "flag-container")]
    pub container_class: &'static str,
    #[props(default = "flag")]
    pub flag_class: &'static str,
    #[props(default = "stripe")]
    pub stripe_class: &'static str,
    #[props(default = "tooltip")]
    pub tooltip_class: &'static str,
}

#[component]
pub fn Flag(props: FlagProps) -> Element {
    let config = match props.r#type.config() {
        Some(cfg) => cfg,
        None => {
            tracing::warn!("Flag configuration not found for {:?}", props.r#type);
            return rsx! {};
        }
    };

    let tooltip_id = format!("tooltip-{}", props.r#type.as_ref());
    let direction = if config.direction == Direction::Horizontal {
        props.horizontal_style
    } else {
        props.vertical_style
    };
    let size = match props.size {
        Size::Small => props.small_style,
        Size::Medium => props.medium_style,
        Size::Large => props.large_style,
    };
    let full_style = format!("{} {} {}", props.style, size, direction);
    let full_class = format!("{} {}", props.flag_class, props.class);

    let mut is_hovered = use_signal(|| false);

    let on_keydown = move |e: Event<KeyboardData>| {
        if e.key() == Key::Enter {
            e.prevent_default();
            tracing::debug!("Selected flag: {}", config.name);
        }
    };

    let tooltip_style = if is_hovered() {
        format!("{} opacity: 1; visibility: visible;", props.tooltip_style)
    } else {
        props.tooltip_style.to_string()
    };

    rsx! {
        div {
            class: "{props.container_class}",
            style: "{props.container_style}",
            div {
                class: "{full_class}",
                style: "{full_style}",
                role: "img",
                aria_label: "{props.aria_label}",
                aria_describedby: "{tooltip_id}",
                aria_roledescription: "flag",
                aria_keyshortcuts: "Enter Space",
                tabindex: "0",
                onmouseover: move |_| is_hovered.set(true),
                onmouseout: move |_| is_hovered.set(false),
                onfocus: move |_| is_hovered.set(true),
                onblur: move |_| is_hovered.set(false),
                onkeydown: on_keydown,
                for (_i, color) in config.colors.iter().enumerate() {
                    div {
                        key: {format!("{}-{}", props.r#type.as_ref(), _i)},
                        class: "{props.stripe_class}",
                        style: format!("{} background-color: {};", props.stripe_style, color),
                        aria_hidden: "true",
                    }
                }
            }
            div {
                id: "{tooltip_id}",
                class: "{props.tooltip_class}",
                role: "tooltip",
                style: "{tooltip_style}",
                "{config.name}"
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct FlagSectionProps {
    #[props(default)]
    pub title: String,
    #[props(default)]
    pub flags: Vec<Type>,
    #[props(default)]
    pub id: &'static str,
    #[props(default = "margin-bottom: 32px;")]
    pub section_style: &'static str,
    #[props(
        default = "font-family: 'SF Pro Text', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 14px; font-weight: 600; color: #333; margin-bottom: 12px; padding-left: 4px;"
    )]
    pub section_title_style: &'static str,
    #[props(
        default = "background-color: #ffffff; border: 2px dashed #7b61ff; border-radius: 8px; padding: 12px; display: flex; flex-wrap: wrap; gap: 8px; align-items: center; min-height: 48px; transition: border-color 0.2s ease;"
    )]
    pub container_style: &'static str,
    #[props(
        default = "color: #666; font-style: italic; font-size: 12px; text-align: center; width: 100%; padding: 16px;"
    )]
    pub empty_state_style: &'static str,
    #[props(default = "section")]
    pub section_class: &'static str,
    #[props(default = "section-title")]
    pub section_title_class: &'static str,
    #[props(default = "flag-container")]
    pub container_class: &'static str,
    #[props(default = "empty-state")]
    pub empty_state_class: &'static str,
}

#[component]
pub fn FlagSection(props: FlagSectionProps) -> Element {
    let heading_id = format!("{}-heading", props.id);
    let description_id = format!("{}-description", props.id);

    rsx! {
        section {
            class: "{props.section_class}",
            style: "{props.section_style}",
            role: "region",
            aria_labelledby: "{heading_id}",
            h2 {
                id: "{heading_id}",
                class: "{props.section_title_class}",
                style: "{props.section_title_style}",
                "{props.title}"
            }
            div {
                class: "{props.container_class}",
                style: "{props.container_style}",
                role: "group",
                aria_labelledby: "{heading_id}",
                aria_describedby: "{description_id}",
                aria_roledescription: "flag group",
                if props.flags.is_empty() {
                    div {
                        id: "{description_id}",
                        class: "{props.empty_state_class}",
                        style: "{props.empty_state_style}",
                        aria_live: "polite",
                        "No flags available in this category"
                    }
                } else {
                    for (_i, flag_type) in props.flags.iter().enumerate() {
                        Flag {
                            key: {format!("{}-{}-{}", props.id, flag_type.as_ref(), _i)},
                            r#type: *flag_type,
                            size: Size::Medium,
                            aria_label: flag_type.as_ref().to_string(),
                            class: "",
                        }
                    }
                }
            }
        }
    }
}
