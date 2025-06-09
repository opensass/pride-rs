use phf::phf_map;
use std::str::FromStr;
use strum_macros::{AsRefStr, Display, EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

/// Enum representing various LGBTQ+ pride flags.
#[derive(
    EnumString, EnumIter, AsRefStr, Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Default,
)]
pub enum Type {
    #[default]
    Rainbow,
    Transgender,
    Bisexual,
    Lesbian,
    Pansexual,
    Asexual,
    NonBinary,
    Aromantic,
    Demisexual,
    Genderfluid,
    Agender,
    Polysexual,
    Omnisexual,
    Demiromantic,
    Graysexual,
}

#[derive(EnumString, EnumIter, AsRefStr, Display, Debug, Clone, Copy, Default, PartialEq)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct FlagConfig {
    pub colors: &'static [&'static str],
    pub direction: Direction,
    pub name: &'static str,
    pub description: &'static str,
}

pub static FLAG_CONFIGURATIONS: phf::Map<&'static str, FlagConfig> = phf_map! {
    "Rainbow" => FlagConfig {
        colors: &["#e40303", "#ff8c00", "#ffed00", "#008018", "#0066ff", "#732982"],
        direction: Direction::Horizontal,
        name: "Pride Rainbow Flag",
        description: "The original rainbow pride flag representing LGBTQ+ community",
    },
    "Transgender" => FlagConfig {
        colors: &["#5bcffa", "#f5abb9", "#ffffff", "#f5abb9", "#5bcffa"],
        direction: Direction::Horizontal,
        name: "Transgender Flag",
        description: "Flag representing transgender community with light blue, pink, and white stripes",
    },
    "Bisexual" => FlagConfig {
        colors: &["#d60270", "#d60270", "#9b59b6", "#0038a8", "#0038a8"],
        direction: Direction::Horizontal,
        name: "Bisexual Flag",
        description: "Flag representing bisexual community with pink, purple, and blue stripes",
    },
    "Lesbian" => FlagConfig {
        colors: &["#d52d00", "#ef7627", "#ff9a56", "#ffffff", "#d162a4", "#b55690", "#a30262"],
        direction: Direction::Horizontal,
        name: "Lesbian Flag",
        description: "Flag representing lesbian community with orange, white, and pink stripes",
    },
    "Pansexual" => FlagConfig {
        colors: &["#ff1b8d", "#ffda00", "#1bb3ff"],
        direction: Direction::Horizontal,
        name: "Pansexual Flag",
        description: "Flag representing pansexual community with pink, yellow, and blue stripes",
    },
    "Asexual" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#810081"],
        direction: Direction::Horizontal,
        name: "Asexual Flag",
        description: "Flag representing asexual community with black, gray, white, and purple stripes",
    },
    "NonBinary" => FlagConfig {
        colors: &["#fcf431", "#fcfcfc", "#9d59d2", "#282828"],
        direction: Direction::Horizontal,
        name: "Non-Binary Flag",
        description: "Flag representing non-binary community with yellow, white, purple, and black stripes",
    },
    "Aromantic" => FlagConfig {
        colors: &["#3ba740", "#a8d47a", "#ffffff", "#ababab", "#000000"],
        direction: Direction::Horizontal,
        name: "Aromantic Flag",
        description: "Flag representing aromantic community with green, light green, white, gray, and black stripes",
    },
    "Demisexual" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#810081"],
        direction: Direction::Horizontal,
        name: "Demisexual Flag",
        description: "Flag representing demisexual community with black, gray, white, and purple stripes",
    },
    "Genderfluid" => FlagConfig {
        colors: &["#ff76a4", "#ffffff", "#c011d7", "#000000", "#303cbe"],
        direction: Direction::Horizontal,
        name: "Genderfluid Flag",
        description: "Flag representing genderfluid community with pink, white, purple, black, and blue stripes",
    },
    "Agender" => FlagConfig {
        colors: &["#000000", "#bababa", "#ffffff", "#b7f684", "#ffffff", "#bababa", "#000000"],
        direction: Direction::Horizontal,
        name: "Agender Flag",
        description: "Flag representing agender community with black, gray, white, and green stripes",
    },
    "Polysexual" => FlagConfig {
        colors: &["#f61cb9", "#07d569", "#1c92f6"],
        direction: Direction::Horizontal,
        name: "Polysexual Flag",
        description: "Flag representing polysexual community with pink, green, and blue stripes",
    },
    "Omnisexual" => FlagConfig {
        colors: &["#ff9ace", "#ff6cab", "#85d7f2", "#67cdf0", "#9378ff"],
        direction: Direction::Horizontal,
        name: "Omnisexual Flag",
        description: "Flag representing omnisexual community with pink, blue, and purple stripes",
    },
    "Demiromantic" => FlagConfig {
        colors: &["#000000", "#a4a4a4", "#ffffff", "#3ba740"],
        direction: Direction::Horizontal,
        name: "Demiromantic Flag",
        description: "Flag representing demiromantic community with black, gray, white, and green stripes",
    },
    "Graysexual" => FlagConfig {
        colors: &["#810081", "#a4a4a4", "#ffffff", "#a4a4a4", "#810081"],
        direction: Direction::Horizontal,
        name: "Graysexual Flag",
        description: "Flag representing graysexual community with purple, gray, and white stripes",
    },
};

pub(crate) fn get_flag_config_by_type(flag_type: Type) -> Option<&'static FlagConfig> {
    FLAG_CONFIGURATIONS.get(flag_type.as_ref())
}
pub(crate) fn get_flag_config_by_str(flag_str: &str) -> Option<&'static FlagConfig> {
    if let Ok(flag_type) = Type::from_str(flag_str) {
        get_flag_config_by_type(flag_type)
    } else {
        None
    }
}

pub trait FlagLookup {
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
