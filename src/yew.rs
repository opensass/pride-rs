// MIT License
//
// Copyright (c) Open SASS Core Maintainers
#![doc = include_str!("../YEW.md")]

use crate::common::Direction;
use crate::common::FlagLookup;
use crate::common::Size;
use crate::common::Type;
use web_sys::KeyboardEvent;
use yew::prelude::*;

/// Props for the [`Flag`] Yew component.
///
/// All props are optional and come with sensible, accessible defaults.  Override
/// only what your design system requires; everything else just works.
///
/// # Feature Gate
///
/// When the `haram` Cargo feature is **disabled** (the default), the variants
/// [`Type::Transgender`], [`Type::NonBinary`], [`Type::Genderfluid`], and
/// [`Type::Agender`] do not exist in the [`Type`] enum.  Pass only the halal
/// variants, or enable the `haram` feature to unlock the full set.
#[derive(Properties, PartialEq, Clone)]
pub struct FlagProps {
    /// The pride flag type to render.  Determines the stripe colors and layout
    /// direction via [`FlagConfig`].
    ///
    /// Defaults to [`Type::Rainbow`].
    #[prop_or_default]
    pub r#type: Type,

    /// Rendered size of the flag.
    ///
    /// Defaults to [`Size::Medium`].
    #[prop_or_default]
    pub size: Size,

    /// Additional CSS class names appended to the flag element.
    ///
    /// Defaults to `""`.
    #[prop_or_default]
    pub class: &'static str,

    /// Accessible label for screen readers (`aria-label`).
    ///
    /// Defaults to an empty string.
    #[prop_or_default]
    pub aria_label: String,

    /// Inline CSS applied to the outermost flag `<div>`.
    ///
    /// Defaults to a flex container with rounded corners, overflow hidden, and a
    /// subtle hover transition.
    #[prop_or(
        "display: flex; border-radius: 4px; overflow: hidden; transition: transform 0.2s ease, box-shadow 0.2s ease; cursor: pointer; position: relative;"
    )]
    pub style: &'static str,

    /// Inline CSS applied when the flag direction is [`Direction::Horizontal`].
    ///
    /// Defaults to `"flex-direction: column;"`.
    #[prop_or("flex-direction: column;")]
    pub horizontal_style: &'static str,

    /// Inline CSS applied when the flag direction is [`Direction::Vertical`].
    ///
    /// Defaults to `"flex-direction: row;"`.
    #[prop_or("flex-direction: row;")]
    pub vertical_style: &'static str,

    /// Inline CSS applied to every individual color stripe `<div>`.
    ///
    /// Defaults to `"flex: 1; min-height: 4px; min-width: 4px;"`.
    #[prop_or("flex: 1; min-height: 4px; min-width: 4px;")]
    pub stripe_style: &'static str,

    /// Inline CSS for the [`Size::Small`] variant.
    ///
    /// Defaults to `"width: 24px; height: 24px;"`.
    #[prop_or("width: 24px; height: 24px;")]
    pub small_style: &'static str,

    /// Inline CSS for the [`Size::Medium`] variant.
    ///
    /// Defaults to `"width: 48px; height: 32px;"`.
    #[prop_or("width: 48px; height: 32px;")]
    pub medium_style: &'static str,

    /// Inline CSS for the [`Size::Large`] variant.
    ///
    /// Defaults to `"width: 96px; height: 64px;"`.
    #[prop_or("width: 96px; height: 64px;")]
    pub large_style: &'static str,

    /// Inline CSS for the wrapper container that holds both the flag and its
    /// tooltip.
    ///
    /// Defaults to `"position: relative; display: inline-block;"`.
    #[prop_or("position: relative; display: inline-block;")]
    pub container_style: &'static str,

    /// Inline CSS for the hover/focus tooltip element.
    ///
    /// Defaults to an absolutely-positioned dark tooltip above the flag.
    #[prop_or(
        "position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); background-color: #333; color: white; padding: 8px 12px; border-radius: 4px; font-size: 12px; white-space: nowrap; transition: opacity 0.2s ease, visibility 0.2s ease; z-index: 1000; pointer-events: none; opacity: 0; visibility: hidden;"
    )]
    pub tooltip_style: &'static str,

    /// CSS class applied to the outer wrapper container.
    ///
    /// Defaults to `"flag-container"`.
    #[prop_or("flag-container")]
    pub container_class: &'static str,

    /// CSS class applied to the main flag element.
    ///
    /// Defaults to `"flag"`.
    #[prop_or("flag")]
    pub flag_class: &'static str,

    /// CSS class applied to each stripe element.
    ///
    /// Defaults to `"stripe"`.
    #[prop_or("stripe")]
    pub stripe_class: &'static str,

    /// CSS class applied to the tooltip element.
    ///
    /// Defaults to `"tooltip"`.
    #[prop_or("tooltip")]
    pub tooltip_class: &'static str,
}

/// Renders a single pride flag as a flex-box grid of colored stripes.
///
/// The component is fully accessible: it exposes `role="img"`, an
/// `aria-label`, a keyboard-focusable container (`tabindex=0`), `Enter`/`Space`
/// key handling, and a linked tooltip via `aria-describedby`.
///
/// # Panics
///
/// Does not panic.  If no [`FlagConfig`] is found for the given [`Type`]
/// (which cannot happen unless the internal map is misconfigured), the component
/// returns an empty fragment and logs a warning.
///
/// # Time Complexity
///
/// O(s) where s is the number of stripes in the flag (≤ 7 in the current data set).
///
/// # Space Complexity
///
/// O(s): one virtual DOM node per stripe.
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

/// Props for the [`FlagSection`] Yew component.
///
/// A section groups multiple flags under a labelled heading, with an accessible
/// empty-state message when no flags are provided.
#[derive(Properties, PartialEq, Clone)]
pub struct FlagSectionProps {
    /// The visible section heading rendered as an `<h2>` element.
    ///
    /// Defaults to an empty string.
    #[prop_or_default]
    pub title: String,

    /// The ordered list of flag types to display inside the section.
    ///
    /// Defaults to an empty vector, which triggers the empty-state UI.
    #[prop_or_default]
    pub flags: Vec<Type>,

    /// A unique identifier used as the prefix for ARIA `labelledby` and
    /// `describedby` attributes.  Must be unique across the page for proper
    /// accessibility.
    ///
    /// Defaults to `""`.
    #[prop_or_default]
    pub id: &'static str,

    /// Inline CSS for the wrapping `<section>` element.
    ///
    /// Defaults to `"margin-bottom: 32px;"`.
    #[prop_or("margin-bottom: 32px;")]
    pub section_style: &'static str,

    /// Inline CSS for the section title `<h2>` element.
    ///
    /// Defaults to a small, semibold, dark-gray label.
    #[prop_or(
        "font-family: 'SF Pro Text', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; font-size: 14px; font-weight: 600; color: #333; margin-bottom: 12px; padding-left: 4px;"
    )]
    pub section_title_style: &'static str,

    /// Inline CSS for the flex container holding the flag elements.
    ///
    /// Defaults to a white dashed-border box with wrap layout.
    #[prop_or(
        "background-color: #ffffff; border: 2px dashed #7b61ff; border-radius: 8px; padding: 12px; display: flex; flex-wrap: wrap; gap: 8px; align-items: center; min-height: 48px; transition: border-color 0.2s ease;"
    )]
    pub container_style: &'static str,

    /// Inline CSS for the empty-state message shown when [`flags`] is empty.
    ///
    /// Defaults to centred, italic, gray text.
    ///
    /// [`flags`]: FlagSectionProps::flags
    #[prop_or(
        "color: #666; font-style: italic; font-size: 12px; text-align: center; width: 100%; padding: 16px;"
    )]
    pub empty_state_style: &'static str,

    /// CSS class for the `<section>` element.
    ///
    /// Defaults to `"section"`.
    #[prop_or("section")]
    pub section_class: &'static str,

    /// CSS class for the `<h2>` title element.
    ///
    /// Defaults to `"section-title"`.
    #[prop_or("section-title")]
    pub section_title_class: &'static str,

    /// CSS class for the flag container `<div>`.
    ///
    /// Defaults to `"flag-container"`.
    #[prop_or("flag-container")]
    pub container_class: &'static str,

    /// CSS class for the empty-state `<div>`.
    ///
    /// Defaults to `"empty-state"`.
    #[prop_or("empty-state")]
    pub empty_state_class: &'static str,
}

/// Renders a titled section containing multiple [`Flag`] components.
///
/// The section uses semantic HTML (`<section>` + `<h2>`) and full ARIA labelling
/// so that assistive technologies can announce the group, its heading, and any
/// empty-state condition.
///
/// # Time Complexity
///
/// O(n) where n is the number of flags in [`FlagSectionProps::flags`].
///
/// # Space Complexity
///
/// O(n): one virtual DOM subtree per flag.
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
