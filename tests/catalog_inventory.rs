mod catalog_inventory {
    pub mod vectors;
}

use catalog_inventory::vectors::{PROPERTY_NEGATIVE_VECTORS, PROPERTY_POSITIVE_VECTORS};
use surgeist_css::{
    CssErrorCode, CssFeatureKind, CssSupportStatus, ErrorKind, feature_metadata,
    parse_style_attribute, property_metadata,
};

const PROPERTY_SUBSET: &str = "The property-specific parser behavior at 4b288d6:src/parser/mod.rs, plus whole-value CSS-wide keywords and syntactically admissible substitution-dependent authored values, is supported.";
const PROPERTY_REMAINDER: &str =
    "Other valid forms of the cited property production are outside the I01 subset.";
const TIMING_SUBSET: &str = "The I01 shorthand components plus C03 duration, signed delay, iteration, and typed calculation syntax are supported.";
const TIMING_REMAINDER: &str = "C05 easing and function grammar closure remains unsupported.";
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
fn public_feature_catalog_exposes_declared_metadata_and_lookup() {
    for vector in PROPERTY_POSITIVE_VECTORS {
        let metadata = property_metadata(vector.canonical_name)
            .unwrap_or_else(|| panic!("missing metadata for `{}`", vector.canonical_name));
        let feature = metadata.feature();

        assert_eq!(feature.id().as_str(), vector.id);
        assert_eq!(feature.kind(), CssFeatureKind::Property);
        assert_eq!(feature.spelling(), vector.canonical_name);
        assert!(
            feature.production().contains("#propdef-"),
            "{} exact property production",
            vector.id
        );
        match vector.id {
            "baseline.property.transition-duration"
            | "baseline.property.transition-delay"
            | "baseline.property.animation-duration"
            | "baseline.property.animation-delay"
            | "baseline.property.animation-iteration-count" => {
                assert_eq!(feature.status(), CssSupportStatus::Complete);
                assert_eq!(feature.supported_subset(), None);
                assert_eq!(feature.unsupported_remainder(), None);
            }
            "baseline.property.transition" | "baseline.property.animation" => {
                assert_eq!(feature.status(), CssSupportStatus::Partial);
                assert_eq!(feature.supported_subset(), Some(TIMING_SUBSET));
                assert_eq!(feature.unsupported_remainder(), Some(TIMING_REMAINDER));
            }
            _ => {
                assert_eq!(feature.status(), CssSupportStatus::Partial);
                assert_eq!(feature.supported_subset(), Some(PROPERTY_SUBSET));
                assert_eq!(feature.unsupported_remainder(), Some(PROPERTY_REMAINDER));
            }
        }
        assert_eq!(feature.recognized_unsupported_code(), None);
        assert_ne!(
            feature.source().id().as_str(),
            "I01-BASE-PARSER",
            "{} retained generic baseline provenance",
            vector.id
        );
        assert_ne!(
            feature.source().url().is_some(),
            feature.source().repository_provenance().is_some(),
            "{} source provenance XOR",
            vector.id
        );
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
    }

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

    let exact_source_cases = [
        ("display", "O-CSS2"),
        ("all", "O-CASCADE4"),
        ("margin", "O-BOX3"),
        ("color", "O-COLOR4"),
        ("background", "O-BACKGROUNDS3"),
        ("font", "O-FONTS3"),
        ("direction", "O-WRITING3"),
        ("flex", "O-FLEXBOX1"),
        ("cursor", "O-UI3"),
        ("transform", "O-TRANSFORMS1"),
        ("overflow-x", "X-OVERFLOW3"),
        ("gap", "S-ALIGN3"),
        ("content-visibility", "I-CONTAIN2"),
        ("counter-set", "I-LISTS3"),
        ("grid-flow-tolerance", "X-GRID-TOLERANCE-BASE"),
        ("grid", "R-GRID2"),
        ("word-break", "S-TEXT3"),
        ("text-wrap", "X-TEXT4"),
        ("text-decoration-line", "S-TEXTDECOR3"),
        ("text-decoration-thickness", "X-TEXTDECOR4"),
        ("inset", "I-POSITION3"),
        ("box-decoration-break", "S-BREAK3"),
        ("order", "S-DISPLAY3"),
        ("aspect-ratio", "I-SIZING3"),
        ("scrollbar-width", "R-SCROLLBARS1"),
        ("user-select", "X-UI4"),
        ("translate", "I-TRANSFORMS2"),
        ("filter", "I-FILTER1"),
        ("backdrop-filter", "X-FILTER2-BASE"),
        ("mask", "S-MASKING1"),
        ("transition", "I-TRANSITIONS1"),
        ("animation", "I-ANIMATIONS1"),
    ];
    for (property, source_id) in exact_source_cases {
        let feature = property_metadata(property)
            .expect("representative property")
            .feature();
        assert_eq!(feature.source().id().as_str(), source_id, "{property}");
    }
}

#[test]
fn authored_property_cases_exercise_public_parser_behavior() {
    for vector in PROPERTY_POSITIVE_VECTORS {
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

    for vector in PROPERTY_NEGATIVE_VECTORS {
        if vector.canonical_name == "all" {
            assert_eq!(vector.authored_value, "block");
        } else {
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
}
