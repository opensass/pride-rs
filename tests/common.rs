// MIT License
//
// Copyright (c) 2026 Open SASS Core Maintainers

use pride_rs::FlagLookup;
use pride_rs::Type;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[test]
fn test_enum_iter_default() {
    let variants: Vec<Type> = Type::iter().collect();
    #[cfg(not(feature = "haram"))]
    assert_eq!(variants.len(), 11);
    #[cfg(feature = "haram")]
    assert_eq!(variants.len(), 15);
}

#[test]
fn test_as_ref_str() {
    assert_eq!(Type::Rainbow.as_ref(), "Rainbow");
    assert_eq!(Type::Bisexual.as_ref(), "Bisexual");
}

#[test]
fn test_display_trait() {
    assert_eq!(Type::Lesbian.to_string(), "Lesbian");
    assert_eq!(Type::Aromantic.to_string(), "Aromantic");
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
fn test_config_haram_field_false_for_halal_type() {
    let config = Type::Rainbow.config().unwrap();
    assert!(!config.haram);
}

#[test]
fn test_config_trait_on_str_invalid() {
    let config = "Unknown".config();
    assert!(config.is_none());
}

#[cfg(feature = "haram")]
mod haram_tests {
    use pride_rs::FlagLookup;
    use pride_rs::Type;
    use std::str::FromStr;

    #[test]
    fn test_is_haram_true_for_transgender() {
        assert!(Type::Transgender.is_haram());
    }

    #[test]
    fn test_is_haram_true_for_nonbinary() {
        assert!(Type::NonBinary.is_haram());
    }

    #[test]
    fn test_is_haram_true_for_genderfluid() {
        assert!(Type::Genderfluid.is_haram());
    }

    #[test]
    fn test_is_haram_true_for_agender() {
        assert!(Type::Agender.is_haram());
    }

    #[test]
    fn test_is_haram_false_for_rainbow() {
        assert!(!Type::Rainbow.is_haram());
    }

    #[test]
    fn test_is_haram_false_for_bisexual() {
        assert!(!Type::Bisexual.is_haram());
    }

    #[test]
    fn test_config_haram_field_true_for_transgender() {
        let config = Type::Transgender.config().unwrap();
        assert!(config.haram);
    }

    #[test]
    fn test_config_haram_field_true_for_nonbinary() {
        let config = Type::NonBinary.config().unwrap();
        assert!(config.haram);
    }

    #[test]
    fn test_haram_str_lookup_transgender() {
        let config = "Transgender".config().unwrap();
        assert_eq!(config.colors[1], "#f5abb9");
    }

    #[test]
    fn test_as_ref_str_haram_variants() {
        assert_eq!(Type::Transgender.as_ref(), "Transgender");
        assert_eq!(Type::NonBinary.as_ref(), "NonBinary");
        assert_eq!(Type::Genderfluid.as_ref(), "Genderfluid");
        assert_eq!(Type::Agender.as_ref(), "Agender");
    }

    #[test]
    fn test_from_str_haram_valid() {
        assert_eq!(Type::from_str("Transgender").unwrap(), Type::Transgender);
        assert_eq!(Type::from_str("Agender").unwrap(), Type::Agender);
    }
}
