use surgeist_css::{
    CssGridMode, CssMediaConditionKind, CssMediaFeatureKind, CssMediaFeatureQuery, CssMediaQuery,
    CssMediaRatio, CssMediaType, CssQueryComparison, CssRatio, CssRecoveryAction,
    CssResolutionUnit, CssRule, CssScanMode, parse_sheet,
};

fn parsed_feature(query: &str) -> CssMediaFeatureQuery {
    let source = format!("@media {query} {{}}");
    let report = parse_sheet(&source);
    assert!(report.is_clean(), "{query}: {:?}", report.diagnostics());
    let [CssRule::Media(rule)] = report.syntax().rules() else {
        panic!("{query}: expected one retained media rule")
    };
    let [CssMediaQuery::Condition(condition)] = rule.query().queries() else {
        panic!("{query}: expected one condition-only query")
    };
    let CssMediaConditionKind::Feature(feature) = condition.kind() else {
        panic!("{query}: expected one media feature")
    };
    feature.clone()
}

fn parsed_type(name: &str) -> CssMediaType {
    let source = format!("@media {name} {{}}");
    let report = parse_sheet(&source);
    assert!(report.is_clean(), "{name}: {:?}", report.diagnostics());
    let [CssRule::Media(rule)] = report.syntax().rules() else {
        panic!("{name}: expected one retained media rule")
    };
    let [CssMediaQuery::Typed(query)] = rule.query().queries() else {
        panic!("{name}: expected one typed query")
    };
    query.media_type()
}

#[test]
fn defined_false_media_syntax_is_not_malformed_recovery() {
    let report = parse_sheet(concat!(
        "@media only future-screen and (unknown-feature: calc(1foo + 2px)), ",
        "(width: calc(1px)), screen {}",
    ));
    assert!(
        report.is_clean(),
        "balanced unknown MQ3 syntax is valid defined-false authored syntax: {:?}",
        report.diagnostics()
    );
    let [CssRule::Media(rule)] = report.syntax().rules() else {
        panic!("expected the media rule to be retained")
    };
    assert_eq!(rule.query().queries().len(), 3);
    assert!(
        rule.query()
            .queries()
            .iter()
            .all(|query| !matches!(query, CssMediaQuery::Never(_))),
        "defined-false members are distinct from malformed recovery"
    );

    for malformed in ["layer", "not", "and", "only", "or", "???", ""] {
        let source = format!("@media screen,{malformed},print {{}} ");
        let report = parse_sheet(&source);
        let [CssRule::Media(rule)] = report.syntax().rules() else {
            panic!("{malformed:?}: expected the media rule to be retained")
        };
        assert!(
            matches!(
                rule.query().queries(),
                [
                    CssMediaQuery::Typed(_),
                    CssMediaQuery::Never(_),
                    CssMediaQuery::Typed(_)
                ]
            ),
            "{malformed:?}: reserved, unexpected, and empty members stay malformed"
        );
        let [diagnostic] = report.diagnostics() else {
            panic!("{malformed:?}: expected exactly one malformed-member diagnostic")
        };
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::ReplaceMediaQueryWithNever,
            "{malformed:?}"
        );
    }

    let scripting = parse_sheet("@media screen,(scripting: enabled),print {}");
    let [CssRule::Media(rule)] = scripting.syntax().rules() else {
        panic!("expected the scripting media rule to be retained")
    };
    assert!(matches!(
        rule.query().queries(),
        [
            CssMediaQuery::Typed(_),
            CssMediaQuery::Never(_),
            CssMediaQuery::Typed(_)
        ]
    ));
    let [diagnostic] = scripting.diagnostics() else {
        panic!("recognized deferred scripting still diagnoses")
    };
    assert_eq!(
        diagnostic.action(),
        CssRecoveryAction::ReplaceMediaQueryWithNever
    );
}

#[test]
fn mq3_named_types_and_features_follow_exact_domains() {
    let report = parse_sheet(concat!(
        "@media speech { .speech { color: red; } } ",
        "@media (device-width: 1px) { .device { color: blue; } }",
    ));

    assert!(
        report.is_clean(),
        "MQ3 speech and device-width are valid authored syntax: {:?}",
        report.diagnostics()
    );

    let [CssRule::Media(speech), CssRule::Media(device)] = report.syntax().rules() else {
        panic!("expected both valid MQ3 media rules to be retained")
    };
    assert!(matches!(
        speech.query().queries(),
        [CssMediaQuery::Typed(_)]
    ));
    assert!(matches!(
        device.query().queries(),
        [CssMediaQuery::Condition(_)]
    ));
}

#[test]
fn mq3_exposes_all_eleven_named_media_types() {
    for (name, expected) in [
        ("all", CssMediaType::All),
        ("aural", CssMediaType::Aural),
        ("braille", CssMediaType::Braille),
        ("embossed", CssMediaType::Embossed),
        ("handheld", CssMediaType::Handheld),
        ("print", CssMediaType::Print),
        ("projection", CssMediaType::Projection),
        ("screen", CssMediaType::Screen),
        ("speech", CssMediaType::Speech),
        ("tty", CssMediaType::Tty),
        ("tv", CssMediaType::Tv),
    ] {
        assert_eq!(parsed_type(name), expected, "{name}");
    }
}

#[test]
fn mq3_boolean_features_preserve_their_typed_names() {
    for (name, expected) in [
        ("width", CssMediaFeatureKind::Width),
        ("height", CssMediaFeatureKind::Height),
        ("device-width", CssMediaFeatureKind::DeviceWidth),
        ("device-height", CssMediaFeatureKind::DeviceHeight),
        ("orientation", CssMediaFeatureKind::Orientation),
        ("aspect-ratio", CssMediaFeatureKind::AspectRatio),
        (
            "device-aspect-ratio",
            CssMediaFeatureKind::DeviceAspectRatio,
        ),
        ("color", CssMediaFeatureKind::Color),
        ("color-index", CssMediaFeatureKind::ColorIndex),
        ("monochrome", CssMediaFeatureKind::Monochrome),
        ("resolution", CssMediaFeatureKind::Resolution),
        ("scan", CssMediaFeatureKind::Scan),
        ("grid", CssMediaFeatureKind::Grid),
    ] {
        let feature = parsed_feature(&format!("({name})"));
        assert_eq!(feature.name(), name);
        assert!(matches!(feature, CssMediaFeatureQuery::Boolean(value) if value == expected));
        assert_eq!(expected.name(), name);
    }
}

#[test]
fn mq3_range_features_follow_length_ratio_integer_and_resolution_domains() {
    assert!(matches!(
        parsed_feature("(device-width: 800px)"),
        CssMediaFeatureQuery::DeviceWidth(value)
            if value.comparison() == Some(CssQueryComparison::Equal)
                && value.value().value().value() == 800.0
    ));
    assert!(matches!(
        parsed_feature("(max-device-height: 7em)"),
        CssMediaFeatureQuery::DeviceHeight(value)
            if value.comparison() == Some(CssQueryComparison::LessThanOrEqual)
                && value.value().value().value() == 7.0
    ));
    assert!(matches!(
        parsed_feature("(width: 0)"),
        CssMediaFeatureQuery::Width(value)
            if value.value().value().value() == 0.0
                && value.value().authored_unit().is_none()
                && value.value().unit() == surgeist_css::CssLengthUnit::Px
    ));
    assert!(matches!(
        parsed_feature("(width: 0px)"),
        CssMediaFeatureQuery::Width(value)
            if value.value().authored_unit() == Some(surgeist_css::CssLengthUnit::Px)
    ));
    assert!(matches!(
        parsed_feature("(min-aspect-ratio: 16/9)"),
        CssMediaFeatureQuery::AspectRatio(value)
            if value.comparison() == Some(CssQueryComparison::GreaterThanOrEqual)
                && value.value().numerator() == 16
                && value.value().denominator() == 9
    ));
    assert!(matches!(
        parsed_feature("(device-aspect-ratio > 4/3)"),
        CssMediaFeatureQuery::DeviceAspectRatio(value)
            if value.comparison() == Some(CssQueryComparison::GreaterThan)
                && value.value().numerator() == 4
                && value.value().denominator() == 3
    ));
    assert!(matches!(
        parsed_feature("(min-color-index: 256)"),
        CssMediaFeatureQuery::ColorIndex(value)
            if value.comparison() == Some(CssQueryComparison::GreaterThanOrEqual)
                && value.value().value() == 256
    ));

    for (css, unit) in [
        ("(resolution: 96dpi)", CssResolutionUnit::Dpi),
        ("(resolution: 38dpcm)", CssResolutionUnit::Dpcm),
        ("(resolution: 2dppx)", CssResolutionUnit::Dppx),
    ] {
        assert!(matches!(
            parsed_feature(css),
            CssMediaFeatureQuery::Resolution(value) if value.value().unit() == unit
        ));
    }
}

#[test]
fn mq3_ratio_is_positive_integer_only_without_narrowing_general_ratio() {
    let ratio = CssMediaRatio::try_new(16, 9).expect("positive MQ3 ratio");
    assert_eq!(ratio.numerator(), 16);
    assert_eq!(ratio.denominator(), 9);
    assert_eq!(CssMediaRatio::try_new(0, 9), None);
    assert_eq!(CssMediaRatio::try_new(16, 0), None);

    let general = CssRatio::try_new(0.0, 1.5).expect("existing general ratio remains broader");
    assert_eq!(general.numerator().value(), 0.0);
    assert_eq!(general.denominator().value(), 1.5);
}

#[test]
fn mq3_scan_and_grid_expose_exact_keyword_and_binary_domains() {
    assert!(matches!(
        parsed_feature("(scan: progressive)"),
        CssMediaFeatureQuery::Scan(CssScanMode::Progressive)
    ));
    assert!(matches!(
        parsed_feature("(scan: interlace)"),
        CssMediaFeatureQuery::Scan(CssScanMode::Interlace)
    ));
    assert!(matches!(
        parsed_feature("(grid: 0)"),
        CssMediaFeatureQuery::Grid(CssGridMode::Bitmap)
    ));
    assert!(matches!(
        parsed_feature("(grid: -0)"),
        CssMediaFeatureQuery::Grid(CssGridMode::Bitmap)
    ));
    assert!(matches!(
        parsed_feature("(grid: 1)"),
        CssMediaFeatureQuery::Grid(CssGridMode::Grid)
    ));
    assert_eq!(CssGridMode::Bitmap.value(), 0);
    assert_eq!(CssGridMode::Grid.value(), 1);
}

#[test]
fn mq3_named_queries_keep_exact_non_bmp_source_positions() {
    let source = "@media /*😀*/ speech, /*😀*/ (device-width: 1px) {}";
    let report = parse_sheet(source);
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::Media(rule)] = report.syntax().rules() else {
        panic!("expected one media rule")
    };
    let [
        CssMediaQuery::Typed(speech),
        CssMediaQuery::Condition(device),
    ] = rule.query().queries()
    else {
        panic!("expected typed speech and device-width condition")
    };

    let speech_offset = source.find("speech").expect("speech position");
    assert_eq!(speech.position().byte_offset().value(), speech_offset);
    assert_eq!(
        speech.position().column().value(),
        u32::try_from(source[..speech_offset].encode_utf16().count()).expect("UTF-16 column")
    );
    let device_offset = source.find("(device-width").expect("device position");
    assert_eq!(device.position().byte_offset().value(), device_offset);
    assert_eq!(
        device.position().column().value(),
        u32::try_from(source[..device_offset].encode_utf16().count()).expect("UTF-16 column")
    );
}

#[test]
fn mq3_invalid_named_feature_domains_recover_one_member_and_retain_siblings() {
    for (invalid, responsible) in [
        ("(min-width)", "min-width"),
        ("(min-orientation: portrait)", "min-orientation"),
        ("(aspect-ratio: 0/1)", "0/1"),
        ("(aspect-ratio: 1.5/1)", "1.5"),
        ("(resolution: 0dpi)", "0dpi"),
        ("(scan: raster)", "raster"),
        ("(grid: 2)", "2"),
    ] {
        let source = format!("@media screen,{invalid},print {{ .x {{ color: red; }} }}");
        let report = parse_sheet(&source);
        let [CssRule::Media(rule)] = report.syntax().rules() else {
            panic!("{invalid}: expected retained media rule")
        };
        assert!(matches!(
            rule.query().queries(),
            [
                CssMediaQuery::Typed(_),
                CssMediaQuery::Never(_),
                CssMediaQuery::Typed(_)
            ]
        ));
        let [diagnostic] = report.diagnostics() else {
            panic!("{invalid}: expected exactly one query diagnostic")
        };
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::ReplaceMediaQueryWithNever,
            "{invalid}"
        );
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            source.find(responsible).expect("responsible token"),
            "{invalid}"
        );

        #[cfg(feature = "app-strict")]
        assert_eq!(
            surgeist_css::validate_sheet(&source)
                .expect_err("strict mode rejects recovered media syntax")
                .diagnostics(),
            report.diagnostics(),
            "{invalid}"
        );
    }
}

#[test]
fn empty_media_list_is_valid_authored_syntax_but_public_construction_stays_checked() {
    let report = parse_sheet("@media {} .after { color: red; }");
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::Media(media), CssRule::Style(_)] = report.syntax().rules() else {
        panic!("expected empty media rule and following style sibling")
    };
    assert!(media.query().queries().is_empty());
    assert!(surgeist_css::CssMediaQueryList::try_new(Vec::new()).is_none());

    #[cfg(feature = "app-strict")]
    assert_eq!(
        surgeist_css::validate_sheet("@media {} .after { color: red; }")
            .expect("empty media list is valid"),
        report.syntax().clone()
    );
}

#[test]
fn mq4_comparisons_and_discrete_features_remain_typed() {
    assert!(matches!(
        parsed_feature("(width >= 600px)"),
        CssMediaFeatureQuery::Width(value)
            if value.comparison() == Some(CssQueryComparison::GreaterThanOrEqual)
    ));
    assert!(matches!(
        parsed_feature("(hover: hover)"),
        CssMediaFeatureQuery::Hover(surgeist_css::CssHoverCapability::Hover)
    ));
    assert!(matches!(
        parsed_feature("(prefers-color-scheme: dark)"),
        CssMediaFeatureQuery::PrefersColorScheme(surgeist_css::CssColorSchemePreference::Dark)
    ));
    assert!(matches!(
        parsed_feature("(display-mode: standalone)"),
        CssMediaFeatureQuery::DisplayMode(surgeist_css::CssDisplayMode::Standalone)
    ));
}
