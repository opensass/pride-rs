# 🧬 Pride RS Dioxus Usage

Adding Pride RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **pride-rs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add pride-rs --features=dio
   ```

1. Import the `Flag` component into your Dioxus application.

## 🛠️ Usage

Each flag type is backed by a configuration that returns a list of colors (stripes) and a direction (horizontal or vertical). The component renders a `<div>` with child `<div>` stripes, each styled with the color and arranged by direction:

```sh
Flag container div (flex layout)
+-------------------------------------------------+
|                                                 |
|  Horizontal direction (flex-direction: column)  |
|  +-------+                                      |
|  | Color |                                      |
|  | Stripe|                                      |
|  +-------+                                      |
|  | Color |                                      |
|  | Stripe|                                      |
|  +-------+                                      |
|  | Color |                                      |
|  | Stripe|                                      |
|  +-------+                                      |
|                                                 |
|                                                 |
|  Vertical direction (flex-direction: row)       |
|  +-------+-------+-------+-------+-------+      |
|  | Color | Color | Color | Color | Color |      |
|  | Stripe| Stripe| Stripe| Stripe| Stripe|      |
|  +-------+-------+-------+-------+-------+      |
|                                                 |
+-------------------------------------------------+
```

The `Direction` enum guides the layout style:

- **Horizontal**: `flex-direction: column;`
- **Vertical**" `flex-direction: row;`

```rust
use dioxus::prelude::*;
use pride_rs::{Size, Type};
use pride_rs::dioxus::{Flag, FlagSection};

#[component]
pub fn App() -> Element {
    rsx! {
        Flag {
            r#type: Type::Bisexual,
        }
        Flag {
            size: Size::Large,
        }
        FlagSection {
            title: "Pride Flags".to_string(),
            flags: vec![
                Type::Rainbow,
                Type::Transgender,
                Type::NonBinary
            ],
            id: "pride"
        }
    }
}
```

## 🔧 Props

### `Flag`

The `Flag` component renders a single flag based on the provided `Type`, with optional sizing, ARIA accessibility labels, hover tooltips, and customizable styles.

#### `FlagProps`

| Prop               | Type           | Description                                           | Default                                                    |
| ------------------ | -------------- | ----------------------------------------------------- | ---------------------------------------------------------- |
| `type`             | `Type`         | Type of the flag. Determines colors and layout.       | `Type::Rainbow                                             |
| `size`             | `Size`         | The size of the flag (`Small`, `Medium`, or `Large`). | `Size::Medium`                                             |
| `class`            | `&'static str` | Additional CSS classes for the flag container.        | `""`                                                       |
| `aria_label`       | `String`       | Accessible label for screen readers.                  | `String::new()`                                            |
| `style`            | `&'static str` | General style applied to the flag.                    | `display: flex; border-radius: 4px; overflow: hidden; ...` |
| `horizontal_style` | `&'static str` | Style for horizontally oriented flags.                | `"flex-direction: column;"`                                |
| `vertical_style`   | `&'static str` | Style for vertically oriented flags.                  | `"flex-direction: row;"`                                   |
| `stripe_style`     | `&'static str` | Style for each individual color stripe.               | `"flex: 1; min-height: 4px; min-width: 4px;"`              |
| `small_style`      | `&'static str` | Dimensions for small flags.                           | `"width: 24px; height: 24px;"`                             |
| `medium_style`     | `&'static str` | Dimensions for medium flags.                          | `"width: 48px; height: 32px;"`                             |
| `large_style`      | `&'static str` | Dimensions for large flags.                           | `"width: 96px; height: 64px;"`                             |
| `container_style`  | `&'static str` | Style for the outer container of the flag.            | `"position: relative; display: inline-block;"`             |
| `tooltip_style`    | `&'static str` | Style for the tooltip shown on hover or focus.        | `"position: absolute; bottom: 100%; left: 50%; ..."`       |
| `container_class`  | `&'static str` | Class name for the outer container.                   | `"flag-container"`                                         |
| `flag_class`       | `&'static str` | Class name for the main flag element.                 | `"flag"`                                                   |
| `stripe_class`     | `&'static str` | Class name for the color stripe elements.             | `"stripe"`                                                 |
| `tooltip_class`    | `&'static str` | Class name for the tooltip.                           | `"tooltip"`                                                |

### `FlagSection`

The `FlagSection` component displays a titled section of multiple flags. It can be used to group flags by type.

#### `FlagSectionProps`

| Prop                  | Type           | Description                                             | Default                                                     |
| --------------------- | -------------- | ------------------------------------------------------- | ----------------------------------------------------------- |
| `title`               | `String`       | The section heading title.                              | `String::new()`                                             |
| `flags`               | `Vec<Type>`    | A list of flag `Type`s to display.                      | `vec![]`                                                    |
| `id`                  | `&'static str` | A unique identifier used for ARIA accessibility labels. | `""`                                                        |
| `section_style`       | `&'static str` | Style for the entire section element.                   | `"margin-bottom: 32px;"`                                    |
| `section_title_style` | `&'static str` | Style for the section's title.                          | `"font-family: 'SF Pro Text', ...; padding-left: 4px;"`     |
| `container_style`     | `&'static str` | Style for the container holding all flag elements.      | `"background-color: #fff; border: 2px dashed #7b61ff; ..."` |
| `empty_state_style`   | `&'static str` | Style for the message shown when `flags` is empty.      | `"color: #666; font-style: italic; ..."`                    |
| `section_class`       | `&'static str` | CSS class for the main section container.               | `"section"`                                                 |
| `section_title_class` | `&'static str` | CSS class for the title.                                | `"section-title"`                                           |
| `container_class`     | `&'static str` | CSS class for the flag container.                       | `"flag-container"`                                          |
| `empty_state_class`   | `&'static str` | CSS class for the empty state message.                  | `"empty-state"`                                             |

## 💡 Notes

- Use `FlagSection` for grouping related flags with semantic markup and accessible labels.
- Use `Flag` alone for isolated flags.
- Customize styles by overriding `style`, `class`, or size props.
- Supports keyboard navigation and tooltips for improved accessibility.
- Each `Flag` renders with `role="img"` and a descriptive `aria-label`.
- Focusable via `tabindex=0`.
- Keyboard operable (`Enter` and `Space` log selection).
- Tooltip linked with `aria-describedby` for screen readers.
- Flags have `aria-roledescription="flag"` for better assistive context.
- `FlagSection` uses `role="group"` with `aria-labelledby` and `aria-describedby`.
- Empty state has polite `aria-live` announcements.
