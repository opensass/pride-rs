// MIT License
//
// Copyright (c) 2026 Open SASS Core Maintainers

#![doc = include_str!("../DIOXUS.md")]

use crate::common::Direction;
use crate::common::FlagLookup;
use crate::common::Size;
use crate::common::Type;
use dioxus::prelude::*;
use dioxus_logger::tracing;

/// Props for the [`Flag`] Dioxus component.
///
/// All props are optional with sensible, accessible defaults.  Only override
/// what your design system requires.
///
/// # Feature Gate
///
/// When the `haram` Cargo feature is **disabled** (the default), the variants
/// [`Type::Transgender`], [`Type::NonBinary`], [`Type::Genderfluid`], and
/// [`Type::Agender`] do not exist in the [`Type`] enum.  Pass only the halal
/// variants, or enable the `haram` feature to unlock the full set.
#[derive(Props, PartialEq, Clone)]
pub struct FlagProps {
    /// The pride flag type to render.  Determines stripe colors and layout direction.
    ///
    /// Defaults to [`Type::Rainbow`].
    #[props(default)]
    pub r#type: Type,

    /// Rendered size of the flag.
    ///
    /// Defaults to [`Size::Medium`].
    #[props(default)]
    pub size: Size,

    /// Additional CSS class names appended to the flag element.
    ///
    /// Defaults to `""`.
    #[props(default)]
    pub class: &'static str,

    /// Accessible label for screen readers (`aria-label`).
    ///
    /// Defaults to an empty string.
    #[props(default)]
    pub aria_label: String,

    /// Inline CSS applied to the outermost flag `<div>`.
    ///
    /// Defaults to a flex container with rounded corners, overflow hidden, and a
    /// subtle hover transition.
    #[props(
        default = "display: flex; border-radius: 4px; overflow: hidden; transition: transform 0.2s ease, box-shadow 0.2s ease; cursor: pointer; position: relative;"
    )]
    pub style: &'static str,

    /// Inline CSS applied when the flag direction is [`Direction::Horizontal`].
    ///
    /// Defaults to `"flex-direction: column;"`.
    #[props(default = "flex-direction: column;")]
    pub horizontal_style: &'static str,

    /// Inline CSS applied when the flag direction is [`Direction::Vertical`].
    ///
    /// Defaults to `"flex-direction: row;"`.
    #[props(default = "flex-direction: row;")]
    pub vertical_style: &'static str,

    /// Inline CSS applied to every individual color stripe `<div>`.
    ///
    /// Defaults to `"flex: 1; min-height: 4px; min-width: 4px;"`.
    #[props(default = "flex: 1; min-height: 4px; min-width: 4px;")]
    pub stripe_style: &'static str,

    /// Inline CSS for the [`Size::Small`] variant.
    ///
    /// Defaults to `"width: 24px; height: 24px;"`.
    #[props(default = "width: 24px; height: 24px;")]
    pub small_style: &'static str,

    /// Inline CSS for the [`Size::Medium`] variant.
    ///
    /// Defaults to `"width: 48px; height: 32px;"`.
    #[props(default = "width: 48px; height: 32px;")]
    pub medium_style: &'static str,

    /// Inline CSS for the [`Size::Large`] variant.
    ///
    /// Defaults to `"width: 96px; height: 64px;"`.
    #[props(default = "width: 96px; height: 64px;")]
    pub large_style: &'static str,

    /// Inline CSS for the wrapper container that holds the flag and its tooltip.
    ///
    /// Defaults to `"position: relative; display: inline-block;"`.
    #[props(default = "position: relative; display: inline-block;")]
    pub container_style: &'static str,

    /// Inline CSS for the hover/focus tooltip element.
    ///
    /// Defaults to an absolutely-positioned dark tooltip above the flag.
    #[props(
        default = "position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); background-color: #333; color: white; padding: 8px 12px; border-radius: 4px; font-size: 12px; white-space: nowrap; transition: opacity 0.2s ease, visibility 0.2s ease; z-index: 1000; pointer-events: none; opacity: 0; visibility: hidden;"
    )]
    pub tooltip_style: &'static str,

    /// CSS class applied to the outer wrapper container.
    ///
    /// Defaults to `"flag-container"`.
    #[props(default = "flag-container")]
    pub container_class: &'static str,

    /// CSS class applied to the main flag element.
    ///
    /// Defaults to `"flag"`.
    #[props(default = "flag")]
    pub flag_class: &'static str,

    /// CSS class applied to each stripe element.
    ///
    /// Defaults to `"stripe"`.
    #[props(default = "stripe")]
    pub stripe_class: &'static str,

    /// CSS class applied to the tooltip element.
    ///
    /// Defaults to `"tooltip"`.
    #[props(default = "tooltip")]
    pub tooltip_class: &'static str,
}

/// Renders a single pride flag as a flex-box grid of colored stripes.
///
/// The component is fully accessible: it exposes `role="img"`, an
/// `aria-label`, a keyboard-focusable container (`tabindex=0`), `Enter`
/// key handling, and a linked tooltip via `aria-describedby`.
///
/// # Panics
///
/// Does not panic.  If no [`FlagConfig`] is found for the given [`Type`]
/// the component returns an empty `rsx!{}` and emits a tracing warning.
///
/// # Time Complexity
///
/// O(s) where s is the number of stripes in the flag (≤ 7 in the current data set).
///
/// # Space Complexity
///
/// O(s): one virtual DOM node per stripe.
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
                        key: "{props.r#type.as_ref()}-{_i}",
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

/// Props for the [`FlagSection`] Dioxus component.
///
/// A section groups multiple flags under a labelled heading, with an accessible
/// empty-state message when no flags are provided.
#[derive(Props, PartialEq, Clone)]
pub struct FlagSectionProps {
    /// The visible section heading rendered as an `<h2>` element.
    ///
    /// Defaults to an empty string.
    #[props(default)]
    pub title: String,

    /// The ordered list of flag types to display inside the section.
    ///
    /// Defaults to an empty vector, which triggers the empty-state UI.
    #[props(default)]
    pub flags: Vec<Type>,

    /// A unique identifier used as the prefix for ARIA `labelledby` and
    /// `describedby` attributes.
    ///
    /// Defaults to `""`.
    #[props(default)]
    pub id: &'static str,

    /// Inline CSS for the wrapping `<section>` element.
    ///
    /// Defaults to `"margin-bottom: 32px;"`.
    #[props(default = "margin-bottom: 32px;")]
    pub section_style: &'static str,

    /// Inline CSS for the section title `<h2>` element.
    ///
    /// Defaults to a small, semibold, dark-gray label.
    #[props(
        default = "font-family: 'SF Pro Text', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 14px; font-weight: 600; color: #333; margin-bottom: 12px; padding-left: 4px;"
    )]
    pub section_title_style: &'static str,

    /// Inline CSS for the flex container holding the flag elements.
    ///
    /// Defaults to a white dashed-border box with wrap layout.
    #[props(
        default = "background-color: #ffffff; border: 2px dashed #7b61ff; border-radius: 8px; padding: 12px; display: flex; flex-wrap: wrap; gap: 8px; align-items: center; min-height: 48px; transition: border-color 0.2s ease;"
    )]
    pub container_style: &'static str,

    /// Inline CSS for the empty-state message shown when [`flags`] is empty.
    ///
    /// Defaults to centred, italic, gray text.
    ///
    /// [`flags`]: FlagSectionProps::flags
    #[props(
        default = "color: #666; font-style: italic; font-size: 12px; text-align: center; width: 100%; padding: 16px;"
    )]
    pub empty_state_style: &'static str,

    /// CSS class for the `<section>` element.
    ///
    /// Defaults to `"section"`.
    #[props(default = "section")]
    pub section_class: &'static str,

    /// CSS class for the `<h2>` title element.
    ///
    /// Defaults to `"section-title"`.
    #[props(default = "section-title")]
    pub section_title_class: &'static str,

    /// CSS class for the flag container `<div>`.
    ///
    /// Defaults to `"flag-container"`.
    #[props(default = "flag-container")]
    pub container_class: &'static str,

    /// CSS class for the empty-state `<div>`.
    ///
    /// Defaults to `"empty-state"`.
    #[props(default = "empty-state")]
    pub empty_state_class: &'static str,
}

/// Renders a titled section containing multiple [`Flag`] components.
///
/// Uses semantic HTML (`<section>` + `<h2>`) and full ARIA labelling so that
/// assistive technologies can announce the group, its heading, and empty-state
/// conditions.
///
/// # Time Complexity
///
/// O(n) where n is the number of flags in [`FlagSectionProps::flags`].
///
/// # Space Complexity
///
/// O(n): one virtual DOM subtree per flag.
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
                            key: "{props.id}-{flag_type.as_ref()}-{_i}",
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
