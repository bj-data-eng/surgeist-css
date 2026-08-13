use surgeist_css::{
    CssErrorCode, CssNamespaceName, CssNamespacePrefix, CssRecoveryAction, CssRule, ErrorKind,
    parse_sheet,
};

#[test]
fn namespace_rules_obey_namespaces3_prelude_ordering() {
    let source = concat!(
        "@namespace \"not a URI\";",
        "@namespace svg url(https://example.test/one);",
        "@namespace SVG \"\";",
        "@namespace s\\76 g \"replacement\";",
        "@namespace \"\";",
    );
    let report = parse_sheet(source);

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [
        CssRule::Namespace(first_default),
        CssRule::Namespace(first_named),
        CssRule::Namespace(case_distinct),
        CssRule::Namespace(replacement),
        CssRule::Namespace(last_default),
    ] = report.syntax().rules()
    else {
        panic!("expected five retained namespace rules in authored order")
    };

    assert!(first_default.prefix().is_none());
    assert_eq!(first_default.name().as_str(), "not a URI");
    assert_eq!(first_named.prefix().expect("named prefix").as_str(), "svg");
    assert_eq!(first_named.name().as_str(), "https://example.test/one");
    assert_eq!(
        case_distinct
            .prefix()
            .expect("case-distinct prefix")
            .as_str(),
        "SVG"
    );
    assert_eq!(case_distinct.name().as_str(), "");
    assert_eq!(
        replacement.prefix().expect("escaped prefix").as_str(),
        "svg"
    );
    assert_eq!(replacement.name().as_str(), "replacement");
    assert!(last_default.prefix().is_none());
    assert_eq!(last_default.name().as_str(), "");
    assert_eq!(first_default.position().byte_offset().value(), 0);
}

#[test]
fn late_namespace_is_a_placement_error_not_an_unsupported_rule() {
    let source = concat!(
        ".before { color: red; } ",
        "@namespace svg url(http://example.test/a;b); ",
        ".after { color: blue; }",
    );
    let report = parse_sheet(source);

    assert!(matches!(
        report.syntax().rules(),
        [CssRule::Style(_), CssRule::Style(_)]
    ));
    let [diagnostic] = report.diagnostics() else {
        panic!("expected one late-namespace recovery diagnostic")
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidAtRulePlacement
    );
    assert_ne!(diagnostic.error().code(), CssErrorCode::UnsupportedAtRule);
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropAtRule);
    let ErrorKind::InvalidAtRulePlacement(detail) = diagnostic.error().kind() else {
        panic!("expected namespace placement payload")
    };
    assert_eq!(detail.name().as_str(), "namespace");
    assert_eq!(
        detail.expected_context().as_str(),
        "after imports and before every layer or body rule"
    );
}

#[test]
fn namespace_public_values_check_prefixes_and_preserve_literal_names() {
    assert_eq!(
        CssNamespacePrefix::try_new("svg")
            .expect("decoded identifier")
            .as_str(),
        "svg"
    );
    assert_eq!(
        CssNamespacePrefix::try_new("SVG")
            .expect("case-sensitive identifier")
            .as_str(),
        "SVG"
    );
    assert!(CssNamespacePrefix::try_new("").is_none());
    assert!(CssNamespacePrefix::try_new("two names").is_none());
    assert!(CssNamespacePrefix::try_new("s\\76 g").is_none());

    assert_eq!(CssNamespaceName::new("").as_str(), "");
    assert_eq!(CssNamespaceName::new("not a URI").as_str(), "not a URI");
}

#[test]
fn malformed_late_and_nested_namespace_rules_drop_one_at_rule_and_keep_siblings() {
    for invalid in [
        "@namespace;",
        "@namespace svg;",
        "@namespace svg ident;",
        "@namespace 123 \"urn:test\";",
        "@namespace \"urn:test\" extra;",
        "@namespace svg url(\"urn:test\") {}",
    ] {
        let source = format!("{invalid} .kept {{ color: red; }}");
        let report = parse_sheet(&source);
        assert!(
            matches!(report.syntax().rules(), [CssRule::Style(_)]),
            "{source}: {:?}",
            report.syntax().rules()
        );
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected one namespace diagnostic")
        };
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropAtRule,
            "{source}"
        );
    }

    let missing_semicolon = parse_sheet("@namespace svg \"urn:test\"");
    assert!(missing_semicolon.syntax().rules().is_empty());
    assert_eq!(missing_semicolon.diagnostics().len(), 1);
    assert_eq!(
        missing_semicolon.diagnostics()[0].action(),
        CssRecoveryAction::DropAtRule
    );

    for source in [
        ".body {} @namespace \"urn:late\"; .kept {}",
        "@media screen { @namespace \"urn:nested\"; .kept {} }",
        ".parent { @namespace \"urn:nested\"; color: red; }",
        "@scope { @namespace \"urn:nested\"; .kept {} }",
    ] {
        let report = parse_sheet(source);
        let diagnostic = report
            .diagnostics()
            .iter()
            .find(|diagnostic| diagnostic.error().code() == CssErrorCode::InvalidAtRulePlacement)
            .unwrap_or_else(|| panic!("{source}: expected namespace placement diagnostic"));
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropAtRule,
            "{source}"
        );
        let ErrorKind::InvalidAtRulePlacement(detail) = diagnostic.error().kind() else {
            panic!("{source}: expected placement payload")
        };
        assert_eq!(detail.name().as_str(), "namespace", "{source}");
        if source.starts_with(".body") {
            assert_eq!(
                detail.expected_context().as_str(),
                "after imports and before every layer or body rule"
            );
        } else {
            assert_eq!(
                detail.expected_context().as_str(),
                "the stylesheet top level"
            );
        }
    }
}

#[test]
fn namespace_qualified_type_universal_and_attribute_selectors_use_active_bindings() {
    let report = parse_sheet(concat!(
        "@namespace \"urn:default\";",
        "@namespace svg \"urn:svg\";",
        "svg|a { color: red; }",
        "svg|* { color: red; }",
        "*|a { color: red; }",
        "|a { color: red; }",
        "a { color: red; }",
        ".attributes[svg|href][*|title][|lang][plain] { color: red; }",
    ));

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().rules().len(), 8);
    assert!(matches!(
        report.syntax().rules(),
        [
            CssRule::Namespace(_),
            CssRule::Namespace(_),
            CssRule::Style(_),
            CssRule::Style(_),
            CssRule::Style(_),
            CssRule::Style(_),
            CssRule::Style(_),
            CssRule::Style(_),
        ]
    ));
}
