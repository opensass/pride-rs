use pride_rs::{FlagLookup, Type};
use std::str::FromStr;
use strum::IntoEnumIterator;

#[test]
fn test_enum_iter() {
    let variants: Vec<Type> = Type::iter().collect();
    assert_eq!(variants.len(), 15);
    assert!(variants.contains(&Type::Bisexual));
    assert!(variants.contains(&Type::Agender));
}

#[test]
fn test_as_ref_str() {
    assert_eq!(Type::Rainbow.as_ref(), "Rainbow");
    assert_eq!(Type::Genderfluid.as_ref(), "Genderfluid");
}

#[test]
fn test_display_trait() {
    assert_eq!(Type::Lesbian.to_string(), "Lesbian");
    assert_eq!(Type::NonBinary.to_string(), "NonBinary");
}

#[test]
fn test_from_str_valid() {
    let parsed = Type::from_str("Omnisexual");
    assert_eq!(parsed.unwrap(), Type::Omnisexual);
}

#[test]
fn test_from_str_invalid() {
    let parsed = Type::from_str("NotAFlag");
    assert!(parsed.is_err());
}

#[test]
fn test_config_trait_on_enum() {
    let config = Type::Polysexual.config().unwrap();
    assert!(config.description.contains("polysexual"));
}

#[test]
fn test_config_trait_on_str_valid() {
    let config = "Transgender".config().unwrap();
    assert_eq!(config.colors[1], "#f5abb9");
}

#[test]
fn test_config_trait_on_str_invalid() {
    let config = "Unknown".config();
    assert!(config.is_none());
}
