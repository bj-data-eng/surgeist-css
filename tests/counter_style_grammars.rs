use surgeist_css::{
    CssCounterStyleDescriptorRef, CssCounterStyleName, CssCounterStyleSystem, CssCounterSymbol,
    CssCounterSymbolIdent, CssErrorCode, CssRecoveryAction, CssRule, CssSupportStatus,
    feature_metadata, parse_sheet,
};

#[test]
fn counter_style_rules_retain_valid_core_definitions() {
    let report = parse_sheet(concat!(
        ".before { color: red; } ",
        "@counter-style cycle { system: cyclic; symbols: ● ○; prefix: 👍; suffix: \" \"; } ",
        "@counter-style digits { system: numeric; symbols: \"0\" \"1\"; } ",
        "@counter-style letters { system: alphabetic; symbols: a b; symbols: x y; } ",
        ".after { color: blue; }",
    ));

    assert!(
        report.is_clean(),
        "valid core counter styles should not recover: {:?}",
        report.diagnostics()
    );
    assert_eq!(report.syntax().rules().len(), 5);
    assert!(
        report
            .diagnostics()
            .iter()
            .all(|diagnostic| diagnostic.error().code() != CssErrorCode::UnsupportedAtRule)
    );

    let CssRule::CounterStyle(cycle) = &report.syntax().rules()[1] else {
        panic!("expected cyclic counter style")
    };
    assert_eq!(cycle.name().as_str(), "cycle");
    assert_eq!(cycle.position().byte_offset().value(), 24);
    assert!(matches!(
        cycle.descriptors().system().map(|value| value.value()),
        Some(CssCounterStyleSystem::Cyclic)
    ));
    let symbols = cycle.descriptors().symbols().expect("cyclic symbols");
    assert!(matches!(
        symbols.symbols(),
        [CssCounterSymbol::Ident(first), CssCounterSymbol::Ident(second)]
            if first.as_str() == "●" && second.as_str() == "○"
    ));
    assert!(matches!(
        cycle.descriptors().prefix().map(|value| value.value()),
        Some(CssCounterSymbol::Ident(value)) if value.as_str() == "👍"
    ));
    assert!(matches!(
        cycle.descriptors().suffix().map(|value| value.value()),
        Some(CssCounterSymbol::String(value)) if value.as_str() == " "
    ));
    assert_eq!(cycle.descriptors().occurrences().count(), 4);

    let CssRule::CounterStyle(digits) = &report.syntax().rules()[2] else {
        panic!("expected numeric counter style")
    };
    assert!(matches!(
        digits.descriptors().system().map(|value| value.value()),
        Some(CssCounterStyleSystem::Numeric)
    ));
    assert_eq!(digits.descriptors().symbols().unwrap().symbols().len(), 2);

    let CssRule::CounterStyle(letters) = &report.syntax().rules()[3] else {
        panic!("expected alphabetic counter style")
    };
    let occurrences = letters.descriptors().occurrences().collect::<Vec<_>>();
    assert!(matches!(
        occurrences.as_slice(),
        [
            CssCounterStyleDescriptorRef::System(_),
            CssCounterStyleDescriptorRef::Symbols(_),
            CssCounterStyleDescriptorRef::Symbols(_),
        ]
    ));
    assert!(matches!(
        letters.descriptors().symbols().unwrap().symbols(),
        [CssCounterSymbol::Ident(first), CssCounterSymbol::Ident(second)]
            if first.as_str() == "x" && second.as_str() == "y"
    ));
}

#[test]
fn counter_style_descriptors_enforce_domains_order_and_recovery() {
    let report = parse_sheet(concat!(
        "@counter-style base { system: cyclic; symbols: b; } ",
        "@counter-style additive { system: additive; negative: \"-\" \"(\"; ",
        "range: infinite -1, 1 infinite; pad: 2 \"0\"; fallback: base; ",
        "additive-symbols: 100 C, 10 X, 1 I, 0 N; speak-as: words; } ",
        "@counter-style inherited { system: extends base; negative: \"(\" \" )\"; ",
        "range: auto; pad: 0 \"\"; fallback: decimal; speak-as: base; }",
    ));

    assert!(
        report.is_clean(),
        "valid descriptor domains should be retained without recovery: {:?}",
        report.diagnostics()
    );
    let [
        CssRule::CounterStyle(base),
        CssRule::CounterStyle(additive),
        CssRule::CounterStyle(inherited),
    ] = report.syntax().rules()
    else {
        panic!("expected all valid counter styles to be retained")
    };
    assert_eq!(base.descriptors().occurrences().count(), 2);
    assert_eq!(additive.descriptors().occurrences().count(), 7);
    assert_eq!(inherited.descriptors().occurrences().count(), 6);
}

#[test]
fn counter_style_system_model_retains_fixed_and_extends_forms() {
    assert!(CssCounterStyleName::try_new("default").is_none());
    assert_eq!(
        CssCounterSymbolIdent::try_new("auto").unwrap().as_str(),
        "auto"
    );
    assert!(CssCounterSymbolIdent::try_new("default").is_none());

    let report = parse_sheet(concat!(
        "@counter-style fixedish { system: fixed -2; symbols: \"a\" \"b\"; } ",
        "@counter-style child { system: extends fixedish; }",
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [
        CssRule::CounterStyle(fixed),
        CssRule::CounterStyle(extended),
    ] = report.syntax().rules()
    else {
        panic!("expected two counter styles")
    };
    assert!(matches!(
        fixed.descriptors().system().map(|value| value.value()),
        Some(CssCounterStyleSystem::Fixed(value)) if value.first_symbol_value() == Some(-2)
    ));
    assert!(matches!(
        extended.descriptors().system().map(|value| value.value()),
        Some(CssCounterStyleSystem::Extends(name)) if name.as_str() == "fixedish"
    ));
    assert!(extended.descriptors().symbols().is_none());
}

#[test]
fn counter_style_descriptor_recovery_keeps_valid_occurrences_and_siblings() {
    let source = concat!(
        ".before { color: red; } ",
        "@counter-style kept { system: cyclic; prefix: \"bad\" \"extra\"; ",
        "symbols: a; mystery: 1; suffix: \"ok\"; } ",
        "@counter-style too-short { system: numeric; symbols: a; } ",
        "@counter-style mixed { system: extends kept; symbols: a; } ",
        "@counter-style missing { system: additive; } ",
        ".after { color: blue; }",
    );
    let report = parse_sheet(source);
    assert!(matches!(
        report.syntax().rules(),
        [
            CssRule::Style(_),
            CssRule::CounterStyle(_),
            CssRule::Style(_)
        ]
    ));
    let CssRule::CounterStyle(kept) = &report.syntax().rules()[1] else {
        panic!("expected retained counter style")
    };
    assert!(kept.descriptors().prefix().is_none());
    assert!(matches!(
        kept.descriptors().suffix().map(|value| value.value()),
        Some(CssCounterSymbol::String(value)) if value.as_str() == "ok"
    ));
    assert_eq!(
        report
            .diagnostics()
            .iter()
            .map(|diagnostic| (diagnostic.error().code(), diagnostic.action()))
            .collect::<Vec<_>>(),
        vec![
            (
                CssErrorCode::InvalidDescriptorValue,
                CssRecoveryAction::DropDescriptor
            ),
            (
                CssErrorCode::UnknownDescriptor,
                CssRecoveryAction::DropDescriptor
            ),
            (
                CssErrorCode::InvalidDescriptorCombination,
                CssRecoveryAction::DropAtRule
            ),
            (
                CssErrorCode::InvalidDescriptorCombination,
                CssRecoveryAction::DropAtRule
            ),
            (
                CssErrorCode::InvalidDescriptorCombination,
                CssRecoveryAction::DropAtRule
            ),
        ]
    );
}

#[test]
fn counter_style_rules_enforce_top_level_body_phase_and_exact_positions() {
    let source = concat!(
        "@counter-style 🧭 { symbols: \"x\"; suffix: \"🚀\"; } ",
        "@import \"late.css\"; ",
        "@media screen { @counter-style nested { symbols: n; } .inside {} } ",
        ".after {}",
    );
    let report = parse_sheet(source);
    assert!(matches!(
        report.syntax().rules(),
        [
            CssRule::CounterStyle(_),
            CssRule::Media(_),
            CssRule::Style(_)
        ]
    ));
    let CssRule::CounterStyle(rule) = &report.syntax().rules()[0] else {
        panic!("expected counter style")
    };
    assert_eq!(rule.name().as_str(), "🧭");
    let suffix_offset = source.find("suffix").unwrap();
    let suffix = rule.descriptors().suffix().unwrap();
    assert_eq!(suffix.position().byte_offset().value(), suffix_offset);
    assert_eq!(
        suffix.position().column().value() as usize,
        source[..suffix_offset].encode_utf16().count()
    );
    assert_eq!(
        report
            .diagnostics()
            .iter()
            .map(|diagnostic| diagnostic.error().code())
            .collect::<Vec<_>>(),
        vec![
            CssErrorCode::InvalidAtRulePlacement,
            CssErrorCode::InvalidAtRulePlacement,
        ]
    );
    let CssRule::Media(media) = &report.syntax().rules()[1] else {
        panic!("expected media rule")
    };
    assert!(matches!(media.rules(), [CssRule::Style(_)]));
}

#[test]
fn counter_style_block_recovery_handles_reserved_names_eof_and_depth_boundaries() {
    for name in ["none", "default", "inherit"] {
        let reserved = parse_sheet(&format!(
            "@counter-style {name} {{ symbols: x; }} .after {{}}"
        ));
        assert!(matches!(reserved.syntax().rules(), [CssRule::Style(_)]));
        assert_eq!(
            reserved.diagnostics()[0].error().code(),
            CssErrorCode::InvalidAtRulePrelude
        );
    }

    let statement = parse_sheet("@counter-style valid; .after {}");
    assert!(matches!(statement.syntax().rules(), [CssRule::Style(_)]));
    assert_eq!(
        statement.diagnostics()[0].error().code(),
        CssErrorCode::InvalidAtRuleBody
    );
    let missing_block = parse_sheet("@counter-style valid");
    assert!(missing_block.syntax().rules().is_empty());
    assert_eq!(
        missing_block.diagnostics()[0].error().code(),
        CssErrorCode::InvalidAtRuleBody
    );

    let eof_source = "@counter-style eof { system: symbolic; symbols: 🚀";
    let eof = parse_sheet(eof_source);
    assert!(matches!(eof.syntax().rules(), [CssRule::CounterStyle(_)]));
    assert_eq!(
        eof.diagnostics()
            .iter()
            .filter(|diagnostic| {
                diagnostic.action() == CssRecoveryAction::RetainWithImplicitClosure
            })
            .count(),
        1
    );

    for depth in [255_usize, 256, 257] {
        let source = format!(
            "@counter-style deep{{system:cyclic;symbols:{}x{};}}.after{{}}",
            "f(".repeat(depth),
            ")".repeat(depth),
        );
        let report = parse_sheet(&source);
        assert!(matches!(
            report.syntax().rules().last(),
            Some(CssRule::Style(_))
        ));
        let stopped = report.diagnostics().iter().any(|diagnostic| {
            diagnostic.error().code() == CssErrorCode::NestingLimit
                && diagnostic.action() == CssRecoveryAction::StopAtNestingLimit
        });
        assert_eq!(
            stopped,
            depth >= 256,
            "depth {depth}: {:?}",
            report.diagnostics()
        );
    }
}

#[test]
fn later_counter_style_rule_metadata_matches_retained_public_behavior() {
    let metadata = feature_metadata("later.rule.counter-style").expect("counter-style metadata");
    assert_eq!(metadata.status(), CssSupportStatus::Complete);
    assert_eq!(metadata.recognized_unsupported_code(), None);
    let report =
        parse_sheet("@counter-style thumbs { system: cyclic; symbols: 👍; suffix: \" \"; }");
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert!(matches!(
        report.syntax().rules(),
        [CssRule::CounterStyle(_)]
    ));
}
