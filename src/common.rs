// MIT License
//
// Copyright (c) 2026 Open SASS Core Maintainers

use phf::phf_map;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

/// Controls the rendered dimensions of a [`Flag`] component.
///
/// Each variant maps to a fixed pair of CSS width/height values on the outermost
/// container element.  The exact pixel values are supplied through the
/// corresponding `*_style` props on the component (`small_style`, `medium_style`,
/// `large_style`).
///
/// # Default
///
/// [`Size::Medium`] is the default variant.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Size {
    /// Compact representation: suitable for icon lists or tight layouts.
    Small,

    /// Standard size: the most common choice; used when no size is specified.
    #[default]
    Medium,

    /// Large representation: useful for hero sections or feature showcases.
    Large,
}

/// Enumerates every pride flag type supported by the crate.
///
/// Each variant corresponds to a static entry in [`FLAG_CONFIGURATIONS`].  The
/// string representation of a variant (via [`AsRefStr`] / [`Display`]) is the
/// key used to look up its [`FlagConfig`].
///
/// Variants guarded by `#[cfg(feature = "haram")]` are **opt-in**: they are
/// compiled into the crate only when the consumer explicitly enables the
/// `haram` Cargo feature.  This allows projects that wish to exclude
/// gender-identity-related flags to do so at compile time with zero runtime
/// overhead.
///
/// # Default
///
/// [`Type::Rainbow`] is the default variant.
///
/// # Feature Gate
///
/// The following variants require the `haram` Cargo feature:
///
/// | Variant | Reason |
/// |---|---|
/// | [`Type::Transgender`] | Gender transition identity |
/// | [`Type::NonBinary`] | Non-binary gender identity |
/// | [`Type::Genderfluid`] | Fluid gender identity |
/// | [`Type::Agender`] | Absence of gender identity |
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum Type {
    /// The original six-stripe rainbow flag: the universal symbol of LGBTQ+
    /// visibility and community solidarity.
    #[default]
    Rainbow,

    /// Flag representing the bisexual community, with pink, purple, and blue stripes.
    Bisexual,

    /// Flag representing the lesbian community, with orange, white, and pink stripes.
    Lesbian,

    /// Flag representing the pansexual community, with pink, yellow, and blue stripes.
    Pansexual,

    /// Flag representing the asexual community, with black, gray, white, and purple stripes.
    Asexual,

    /// Flag representing the aromantic community, with green, white, gray, and black stripes.
    Aromantic,

    /// Flag representing the demisexual community, with black, gray, white, and purple stripes.
    Demisexual,

    /// Flag representing the polysexual community, with pink, green, and blue stripes.
    Polysexual,

    /// Flag representing the omnisexual community, with pink, blue, and purple stripes.
    Omnisexual,

    /// Flag representing the demiromantic community, with black, gray, white, and green stripes.
    Demiromantic,

    /// Flag representing the graysexual community, with purple, gray, and white stripes.
    Graysexual,

    /// Flag representing the transgender community, with light blue, pink, and white stripes.
    ///
    /// # Feature Gate
    ///
    /// Requires the `haram` Cargo feature to be enabled:
    ///
    /// ```toml
    /// pride-rs = { version = "0.1.0", features = ["haram"] }
    /// ```
    #[cfg(feature = "haram")]
    Transgender,

    /// Flag representing the non-binary community, with yellow, white, purple, and black stripes.
    ///
    /// # Feature Gate
    ///
    /// Requires the `haram` Cargo feature to be enabled:
    ///
    /// ```toml
    /// pride-rs = { version = "0.1.0", features = ["haram"] }
    /// ```
    #[cfg(feature = "haram")]
    NonBinary,

    /// Flag representing the genderfluid community, with pink, white, purple, black, and blue stripes.
    ///
    /// # Feature Gate
    ///
    /// Requires the `haram` Cargo feature to be enabled:
    ///
    /// ```toml
    /// pride-rs = { version = "0.1.0", features = ["haram"] }
    /// ```
    #[cfg(feature = "haram")]
    Genderfluid,

    /// Flag representing the agender community, with black, gray, white, and green stripes.
    ///
    /// # Feature Gate
    ///
    /// Requires the `haram` Cargo feature to be enabled:
    ///
    /// ```toml
    /// pride-rs = { version = "0.1.0", features = ["haram"] }
    /// ```
    #[cfg(feature = "haram")]
    Agender,
}

impl Type {
    /// Returns `true` if this flag type is classified as *haram*: i.e., it is
    /// associated with gender-identity change or gender non-conformity.
    ///
    /// The four haram variants are:
    /// - [`Type::Transgender`]
    /// - [`Type::NonBinary`]
    /// - [`Type::Genderfluid`]
    /// - [`Type::Agender`]
    ///
    /// This method is only compiled when the `haram` Cargo feature is enabled,
    /// because the variants themselves are also guarded behind that feature.
    ///
    /// # Returns
    ///
    /// `true` if the variant requires the `haram` feature, `false` otherwise.
    ///
    /// # Time Complexity
    ///
    /// O(1): simple pattern match with no heap allocation.
    ///
    /// # Space Complexity
    ///
    /// O(1): no auxiliary storage.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[cfg(feature = "haram")]
    /// # {
    /// use pride_rs::Type;
    ///
    /// assert!(Type::Transgender.is_haram());
    /// assert!(Type::NonBinary.is_haram());
    /// assert!(!Type::Rainbow.is_haram());
    /// assert!(!Type::Bisexual.is_haram());
    /// # }
    /// ```
    #[cfg(feature = "haram")]
    pub fn is_haram(self) -> bool {
        matches!(
            self,
            Type::Transgender | Type::NonBinary | Type::Genderfluid | Type::Agender
        )
    }
}

/// Controls whether flag stripes are stacked vertically or laid out side-by-side.
///
/// The `Direction` maps directly to a CSS `flex-direction` value applied to the
/// outermost flag container.
///
/// # Default
///
/// [`Direction::Horizontal`] is the default variant.
#[derive(EnumString, EnumIter, AsRefStr, Display, Debug, Clone, Copy, Default, PartialEq)]
pub enum Direction {
    /// Stripes are rendered as horizontal bands stacked top-to-bottom
    /// (`flex-direction: column`).
    #[default]
    Horizontal,

    /// Stripes are rendered as vertical columns laid out left-to-right
    /// (`flex-direction: row`).
    Vertical,
}

/// Compile-time configuration for a single pride flag.
///
/// Each instance describes the complete visual specification of one flag.
/// Instances are stored in the [`FLAG_CONFIGURATIONS`] static map and looked up
/// by the string representation of a [`Type`] variant.
///
/// # Lifetime
///
/// All fields use `'static` string slices, enabling zero-copy embedding of flag
/// data directly in the binary with no heap allocation.
#[derive(Debug)]
pub struct FlagConfig {
    /// An ordered slice of CSS hex color strings (`"#rrggbb"`) representing the
    /// flag's stripes, from top-to-bottom (horizontal) or left-to-right (vertical).
    pub colors: &'static [&'static str],

    /// Whether the stripes run horizontally or vertically.
    pub direction: Direction,

    /// Human-readable display name for the flag, used in tooltips and ARIA labels.
    pub name: &'static str,

    /// A short prose description of the flag's community symbolism, used in
    /// screen-reader contexts and documentation.
    pub description: &'static str,

    /// `true` if this flag type is guarded by the `haram` Cargo feature.
    ///
    /// This field is always present in the struct (regardless of whether the
    /// `haram` feature is enabled) so that tooling and documentation generators
    /// can inspect gate status at runtime without conditional compilation.
    pub haram: bool,
}

/// Build-time perfect hash map from [`Type`] string key to its [`FlagConfig`].
///
/// The map is constructed at compile time using the [`phf`] crate, guaranteeing
/// O(1) worst-case lookup with zero runtime initialization cost.
///
/// When the `haram` Cargo feature is **disabled** (the default), the four
/// gender-identity-related entries (`Transgender`, `NonBinary`, `Genderfluid`,
/// `Agender`) are absent from the map.  When the feature is **enabled**, the
/// complete set of fifteen entries is present.
///
/// # Time Complexity
///
/// O(1) per lookup: perfect hash with no collision chains.
///
/// # Space Complexity
///
/// O(n) where n is the number of flag entries (11 or 15 depending on feature flags).
#[cfg(not(feature = "haram"))]
pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    "Rainbow" => FlagConfig {
        colors: &["#e40303", "#ff8c00", "#ffed00", "#008018", "#0066ff", "#732982"],
        direction: Direction::Horizontal,
        name: "Pride Rainbow Flag",
        description: "The original rainbow pride flag representing LGBTQ+ community",
        haram: false,
    },
    "Bisexual" => FlagConfig {
        colors: &["#d60270", "#d60270", "#9b59b6", "#0038a8", "#0038a8"],
        direction: Direction::Horizontal,
        name: "Bisexual Flag",
        description: "Flag representing bisexual community with pink, purple, and blue stripes",
        haram: false,
    },
    "Lesbian" => FlagConfig {
        colors: &["#d52d00", "#ef7627", "#ff9a56", "#ffffff", "#d162a4", "#b55690", "#a30262"],
        direction: Direction::Horizontal,
        name: "Lesbian Flag",
        description: "Flag representing lesbian community with orange, white, and pink stripes",
        haram: false,
    },
    "Pansexual" => FlagConfig {
        colors: &["#ff1b8d", "#ffda00", "#1bb3ff"],
        direction: Direction::Horizontal,
        name: "Pansexual Flag",
        description: "Flag representing pansexual community with pink, yellow, and blue stripes",
        haram: false,
    },
    "Asexual" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#810081"],
        direction: Direction::Horizontal,
        name: "Asexual Flag",
        description: "Flag representing asexual community with black, gray, white, and purple stripes",
        haram: false,
    },
    "Aromantic" => FlagConfig {
        colors: &["#3ba740", "#a8d47a", "#ffffff", "#ababab", "#000000"],
        direction: Direction::Horizontal,
        name: "Aromantic Flag",
        description: "Flag representing aromantic community with green, light green, white, gray, and black stripes",
        haram: false,
    },
    "Demisexual" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#810081"],
        direction: Direction::Horizontal,
        name: "Demisexual Flag",
        description: "Flag representing demisexual community with black, gray, white, and purple stripes",
        haram: false,
    },
    "Polysexual" => FlagConfig {
        colors: &["#f61cb9", "#07d569", "#1c92f6"],
        direction: Direction::Horizontal,
        name: "Polysexual Flag",
        description: "Flag representing polysexual community with pink, green, and blue stripes",
        haram: false,
    },
    "Omnisexual" => FlagConfig {
        colors: &["#ff9ace", "#ff6cab", "#85d7f2", "#67cdf0", "#9378ff"],
        direction: Direction::Horizontal,
        name: "Omnisexual Flag",
        description: "Flag representing omnisexual community with pink, blue, and purple stripes",
        haram: false,
    },
    "Demiromantic" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#3ba740"],
        direction: Direction::Horizontal,
        name: "Demiromantic Flag",
        description: "Flag representing demiromantic community with black, gray, white, and green stripes",
        haram: false,
    },
    "Graysexual" => FlagConfig {
        colors: &["#810081", "#a4a4a4", "#ffffff", "#a4a4a4", "#810081"],
        direction: Direction::Horizontal,
        name: "Graysexual Flag",
        description: "Flag representing graysexual community with purple, gray, and white stripes",
        haram: false,
    },
};

/// Build-time perfect hash map from [`Type`] string key to its [`FlagConfig`].
///
/// This variant of the map is compiled when the `haram` Cargo feature is
/// **enabled**, and includes all fifteen flag types: including the four
/// gender-identity-related entries gated behind that feature.
///
/// # Time Complexity
///
/// O(1) per lookup: perfect hash with no collision chains.
///
/// # Space Complexity
///
/// O(n) where n = 15 (all supported flag types).
#[cfg(feature = "haram")]
pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    "Rainbow" => FlagConfig {
        colors: &["#e40303", "#ff8c00", "#ffed00", "#008018", "#0066ff", "#732982"],
        direction: Direction::Horizontal,
        name: "Pride Rainbow Flag",
        description: "The original rainbow pride flag representing LGBTQ+ community",
        haram: false,
    },
    "Bisexual" => FlagConfig {
        colors: &["#d60270", "#d60270", "#9b59b6", "#0038a8", "#0038a8"],
        direction: Direction::Horizontal,
        name: "Bisexual Flag",
        description: "Flag representing bisexual community with pink, purple, and blue stripes",
        haram: false,
    },
    "Lesbian" => FlagConfig {
        colors: &["#d52d00", "#ef7627", "#ff9a56", "#ffffff", "#d162a4", "#b55690", "#a30262"],
        direction: Direction::Horizontal,
        name: "Lesbian Flag",
        description: "Flag representing lesbian community with orange, white, and pink stripes",
        haram: false,
    },
    "Pansexual" => FlagConfig {
        colors: &["#ff1b8d", "#ffda00", "#1bb3ff"],
        direction: Direction::Horizontal,
        name: "Pansexual Flag",
        description: "Flag representing pansexual community with pink, yellow, and blue stripes",
        haram: false,
    },
    "Asexual" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#810081"],
        direction: Direction::Horizontal,
        name: "Asexual Flag",
        description: "Flag representing asexual community with black, gray, white, and purple stripes",
        haram: false,
    },
    "Aromantic" => FlagConfig {
        colors: &["#3ba740", "#a8d47a", "#ffffff", "#ababab", "#000000"],
        direction: Direction::Horizontal,
        name: "Aromantic Flag",
        description: "Flag representing aromantic community with green, light green, white, gray, and black stripes",
        haram: false,
    },
    "Demisexual" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#810081"],
        direction: Direction::Horizontal,
        name: "Demisexual Flag",
        description: "Flag representing demisexual community with black, gray, white, and purple stripes",
        haram: false,
    },
    "Polysexual" => FlagConfig {
        colors: &["#f61cb9", "#07d569", "#1c92f6"],
        direction: Direction::Horizontal,
        name: "Polysexual Flag",
        description: "Flag representing polysexual community with pink, green, and blue stripes",
        haram: false,
    },
    "Omnisexual" => FlagConfig {
        colors: &["#ff9ace", "#ff6cab", "#85d7f2", "#67cdf0", "#9378ff"],
        direction: Direction::Horizontal,
        name: "Omnisexual Flag",
        description: "Flag representing omnisexual community with pink, blue, and purple stripes",
        haram: false,
    },
    "Demiromantic" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#3ba740"],
        direction: Direction::Horizontal,
        name: "Demiromantic Flag",
        description: "Flag representing demiromantic community with black, gray, white, and green stripes",
        haram: false,
    },
    "Graysexual" => FlagConfig {
        colors: &["#810081", "#a4a4a4", "#ffffff", "#a4a4a4", "#810081"],
        direction: Direction::Horizontal,
        name: "Graysexual Flag",
        description: "Flag representing graysexual community with purple, gray, and white stripes",
        haram: false,
    },
    "Transgender" => FlagConfig {
        colors: &["#5bcffa", "#f5abb9", "#ffffff", "#f5abb9", "#5bcffa"],
        direction: Direction::Horizontal,
        name: "Transgender Flag",
        description: "Flag representing transgender community with light blue, pink, and white stripes",
        haram: true,
    },
    "NonBinary" => FlagConfig {
        colors: &["#fcf431", "#fcfcfc", "#9d59d2", "#282828"],
        direction: Direction::Horizontal,
        name: "Non-Binary Flag",
        description: "Flag representing non-binary community with yellow, white, purple, and black stripes",
        haram: true,
    },
    "Genderfluid" => FlagConfig {
        colors: &["#ff76a4", "#ffffff", "#c011d7", "#000000", "#303cbe"],
        direction: Direction::Horizontal,
        name: "Genderfluid Flag",
        description: "Flag representing genderfluid community with pink, white, purple, black, and blue stripes",
        haram: true,
    },
    "Agender" => FlagConfig {
        colors: &["#000000", "#bababa", "#ffffff", "#b7f684", "#ffffff", "#bababa", "#000000"],
        direction: Direction::Horizontal,
        name: "Agender Flag",
        description: "Flag representing agender community with black, gray, white, and green stripes",
        haram: true,
    },
};

/// Looks up the [`FlagConfig`] for the given [`Type`] variant in [`FLAG_CONFIGURATIONS`].
///
/// Returns `None` only when the `haram` feature is disabled and `flag_type` is
/// one of the four guarded variants: which in practice cannot happen because
/// those variants do not exist in the enum without the feature.  The return type
/// is `Option` for forward-compatibility and API consistency with
/// [`get_flag_config_by_str`].
///
/// # Arguments
///
/// * `flag_type`: a [`Type`] variant whose string form serves as the map key.
///
/// # Returns
///
/// `Some(&FlagConfig)` on a successful lookup, `None` if the key is absent.
///
/// # Time Complexity
///
/// O(1): single perfect-hash map lookup.
///
/// # Space Complexity
///
/// O(1): no heap allocation.
pub(crate) fn get_flag_config_by_type(flag_type: Type) -> Option<&'static FlagConfig> {
    FLAG_CONFIGURATIONS.get(flag_type.as_ref())
}

/// Parses `flag_str` into a [`Type`] then delegates to [`get_flag_config_by_type`].
///
/// This is a convenience wrapper that allows string-keyed lookups from dynamic
/// sources (e.g., user input, serialized state) without the caller needing to
/// import `FromStr`.
///
/// # Arguments
///
/// * `flag_str`: a string slice that must exactly match a [`Type`] variant name
///   (case-sensitive, e.g. `"Rainbow"`, `"Bisexual"`).
///
/// # Returns
///
/// `Some(&FlagConfig)` if `flag_str` is a valid variant name present in the map,
/// `None` otherwise (invalid string or haram variant when feature is disabled).
///
/// # Time Complexity
///
/// O(k) where k is the length of `flag_str` for the string parse, O(1) for the
/// subsequent map lookup.
///
/// # Space Complexity
///
/// O(1): no heap allocation.
pub(crate) fn get_flag_config_by_str(flag_str: &str) -> Option<&'static FlagConfig> {
    Type::from_str(flag_str)
        .ok()
        .and_then(get_flag_config_by_type)
}

/// Unified interface for retrieving a flag's static [`FlagConfig`].
///
/// Implemented by both [`Type`] (enum variant lookup) and `&str` (string lookup).
/// The trait abstracts over the two call sites in the rendering components,
/// keeping component code generic and free of conditional branches.
pub trait FlagLookup {
    /// Returns the [`FlagConfig`] associated with this value, or `None` if no
    /// configuration is registered.
    ///
    /// # Time Complexity
    ///
    /// O(1) for [`Type`] variants; O(k) parse + O(1) lookup for `&str`.
    ///
    /// # Space Complexity
    ///
    /// O(1): returns a reference into the static map.
    fn config(&self) -> Option<&'static FlagConfig>;
}

impl FlagLookup for Type {
    fn config(&self) -> Option<&'static FlagConfig> {
        get_flag_config_by_type(*self)
    }
}

impl FlagLookup for &str {
    fn config(&self) -> Option<&'static FlagConfig> {
        get_flag_config_by_str(self)
    }
}
