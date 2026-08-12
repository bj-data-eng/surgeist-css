use std::collections::HashSet;

mod catalog_inventory {
    pub mod vectors;
}

use catalog_inventory::vectors::{PROPERTY_NEGATIVE_VECTORS, PROPERTY_POSITIVE_VECTORS};
use surgeist_css::{
    CssErrorCode, CssFeatureKind, CssSupportStatus, ErrorKind, feature_catalog, feature_metadata,
    parse_style_attribute, property_metadata,
};

const PROPERTY_SUBSET: &str = "The property-specific parser behavior at 4b288d6:src/parser/mod.rs, plus whole-value CSS-wide keywords and syntactically admissible substitution-dependent authored values, is supported.";
const PROPERTY_REMAINDER: &str =
    "Other valid forms of the cited property production are outside the I01 subset.";
const CSS_WIDE_KEYWORDS: &[&str] = &["inherit", "initial", "unset", "revert", "revert-layer"];

fn starts_with_css_wide_keyword(authored_value: &str) -> bool {
    authored_value
        .trim_start()
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .next()
        .is_some_and(|first| {
            CSS_WIDE_KEYWORDS
                .iter()
                .any(|keyword| first.eq_ignore_ascii_case(keyword))
        })
}

fn contains_substitution(authored_value: &str) -> bool {
    authored_value.to_ascii_lowercase().contains("var(")
}

#[test]
fn catalog_inventory_has_exact_independent_property_records_and_lookup() {
    assert_eq!(feature_catalog().len(), 219);
    assert_eq!(PROPERTY_POSITIVE_VECTORS.len(), 179);
    assert_eq!(PROPERTY_NEGATIVE_VECTORS.len(), 179);

    let property_features: Vec<_> = feature_catalog()
        .iter()
        .filter(|feature| feature.kind() == CssFeatureKind::Property)
        .collect();
    assert_eq!(property_features.len(), 179);
    assert_eq!(feature_catalog().len() - property_features.len(), 40);

    let mut all_ids = HashSet::new();
    for feature in feature_catalog() {
        assert!(
            all_ids.insert(feature.id().as_str()),
            "duplicate catalog ID `{}`",
            feature.id().as_str()
        );
    }
    assert_eq!(all_ids.len(), 219);

    let mut names_and_aliases = HashSet::new();
    for vector in PROPERTY_POSITIVE_VECTORS {
        let metadata = property_metadata(vector.canonical_name)
            .unwrap_or_else(|| panic!("missing metadata for `{}`", vector.canonical_name));
        let feature = metadata.feature();

        assert_eq!(feature.id().as_str(), vector.id);
        assert_eq!(feature.kind(), CssFeatureKind::Property);
        assert_eq!(feature.spelling(), vector.canonical_name);
        assert_eq!(feature.production(), vector.canonical_name);
        assert_eq!(feature.status(), CssSupportStatus::Partial);
        assert_eq!(feature.supported_subset(), Some(PROPERTY_SUBSET));
        assert_eq!(feature.unsupported_remainder(), Some(PROPERTY_REMAINDER));
        assert_eq!(feature.recognized_unsupported_code(), None);
        assert_eq!(
            feature.source().repository_provenance(),
            Some("4b288d6:src/parser/mod.rs")
        );
        assert_eq!(feature.source().url(), None);
        assert_eq!(metadata.property().canonical_name(), vector.canonical_name);
        assert_eq!(metadata.canonical_name(), vector.canonical_name);
        assert!(metadata.aliases().is_empty());
        assert!(std::ptr::eq(
            feature,
            feature_metadata(vector.id).expect("exact feature lookup")
        ));

        let folded = vector.canonical_name.to_ascii_uppercase();
        assert_eq!(
            property_metadata(&folded).map(|entry| entry.property()),
            Some(metadata.property())
        );

        assert!(
            names_and_aliases.insert(vector.canonical_name.to_ascii_lowercase()),
            "duplicate property name `{}`",
            vector.canonical_name
        );
        for alias in metadata.aliases() {
            assert!(
                names_and_aliases.insert(alias.to_ascii_lowercase()),
                "duplicate property alias `{alias}`"
            );
        }
    }

    assert_eq!(names_and_aliases.len(), 179);
    for name in [
        "--display",
        "--custom",
        "definitely-unknown",
        "",
        " display",
    ] {
        assert!(
            property_metadata(name).is_none(),
            "unexpected metadata for `{name}`"
        );
    }
    assert!(feature_metadata("BASELINE.PROPERTY.DISPLAY").is_none());
}

#[test]
fn catalog_inventory_positive_and_negative_manifests_exercise_every_property() {
    let mut positive_ids = HashSet::new();
    for vector in PROPERTY_POSITIVE_VECTORS {
        assert!(
            positive_ids.insert(vector.id),
            "duplicate positive `{}`",
            vector.id
        );
        if vector.canonical_name == "all" {
            assert!(
                CSS_WIDE_KEYWORDS
                    .iter()
                    .any(|keyword| vector.authored_value.trim().eq_ignore_ascii_case(keyword))
                    || contains_substitution(vector.authored_value),
                "`all` positive must use its valid global/substitution contract"
            );
        } else {
            assert!(
                !starts_with_css_wide_keyword(vector.authored_value),
                "{} positive must reach property-specific dispatch",
                vector.id
            );
            assert!(
                !contains_substitution(vector.authored_value),
                "{} positive must not use substitution-dependent parsing",
                vector.id
            );
        }
        let report = parse_style_attribute(&format!(
            "{}: {}",
            vector.canonical_name, vector.authored_value
        ));
        let (declarations, diagnostics) = report.into_parts();
        assert!(
            diagnostics.is_empty(),
            "{} positive vector produced {diagnostics:?}",
            vector.id
        );
        let [declaration] = declarations.as_slice() else {
            panic!(
                "{} positive vector did not retain one declaration",
                vector.id
            );
        };
        let known = declaration
            .known()
            .unwrap_or_else(|| panic!("{} positive was not a known declaration", vector.id));
        assert_eq!(known.property().canonical_name(), vector.canonical_name);
        assert_eq!(known.property().stable_id(), vector.id);
    }

    let mut negative_ids = HashSet::new();
    for vector in PROPERTY_NEGATIVE_VECTORS {
        assert!(
            negative_ids.insert(vector.id),
            "duplicate negative `{}`",
            vector.id
        );
        let positive = PROPERTY_POSITIVE_VECTORS
            .iter()
            .find(|positive| positive.id == vector.id)
            .unwrap_or_else(|| panic!("{} negative has no positive peer", vector.id));
        assert_eq!(positive.canonical_name, vector.canonical_name);
        if vector.canonical_name == "all" {
            assert_eq!(vector.authored_value, "block");
        } else {
            assert_ne!(
                vector.authored_value, positive.authored_value,
                "{} negative must differ from its accepted typed value",
                vector.id
            );
            assert!(
                !vector.authored_value.trim_end().ends_with('/'),
                "{} negative must use a property-specific rejection, not shared trailing syntax",
                vector.id
            );
        }
        assert!(
            !starts_with_css_wide_keyword(vector.authored_value),
            "{} negative must reach property-specific dispatch",
            vector.id
        );
        assert!(
            !contains_substitution(vector.authored_value),
            "{} negative must not use substitution-dependent parsing",
            vector.id
        );
        let report = parse_style_attribute(&format!(
            "{}: {}",
            vector.canonical_name, vector.authored_value
        ));
        let (declarations, diagnostics) = report.into_parts();
        assert!(
            declarations.is_empty(),
            "{} negative vector was retained",
            vector.id
        );
        assert_eq!(diagnostics.len(), 1, "{} negative diagnostics", vector.id);
        let error = diagnostics[0].error();
        assert_eq!(
            error.code(),
            CssErrorCode::InvalidPropertyValue,
            "{} negative diagnostic root",
            vector.id
        );
        let ErrorKind::InvalidPropertyValue(detail) = error.kind() else {
            panic!("{} negative returned {error:?}", vector.id);
        };
        assert_eq!(detail.property().canonical_name(), vector.canonical_name);
        assert_eq!(detail.property().stable_id(), vector.id);
    }

    assert_eq!(positive_ids.len(), 179);
    assert_eq!(negative_ids.len(), 179);
    assert_eq!(positive_ids, negative_ids);
}
