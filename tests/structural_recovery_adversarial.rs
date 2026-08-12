use std::panic::{AssertUnwindSafe, catch_unwind};

use surgeist_css::{
    CssErrorCode, CssRecoveryAction, CssRule, CssScopedRule, ErrorKind, parse_sheet,
};

fn nested_layers(depth: usize, tail: &str) -> String {
    let mut source = "@layer{".repeat(depth);
    source.push_str(&"}".repeat(depth));
    source.push_str(tail);
    source
}

fn component_value(total_depth: usize, opener: &str, closer: &str) -> String {
    let component_depth = total_depth.saturating_sub(1);
    format!(
        ".target{{--x:{}x{};color:blue}}.after{{color:red}}",
        opener.repeat(component_depth),
        closer.repeat(component_depth),
    )
}

fn malformed_block(prefix: &str, total_depth: usize, tail: &str) -> String {
    let component_depth = total_depth.saturating_sub(1);
    format!(
        "{prefix}{}x{};}}{tail}",
        "f(".repeat(component_depth),
        ")".repeat(component_depth),
    )
}

fn nesting_detail(
    report: &surgeist_css::CssParseReport<surgeist_css::CssSheet>,
) -> (
    &surgeist_css::CssNestingLimitError,
    surgeist_css::CssSourceSpan,
) {
    let [diagnostic] = report.diagnostics() else {
        panic!(
            "expected exactly one nesting-limit diagnostic: {:?}",
            report.diagnostics()
        );
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::NestingLimit);
    assert_eq!(diagnostic.action(), CssRecoveryAction::StopAtNestingLimit);
    let ErrorKind::NestingLimit(detail) = diagnostic.error().kind() else {
        panic!("expected typed nesting-limit detail");
    };
    (detail, diagnostic.span())
}

#[test]
fn structural_recovery_accepts_256_rule_blocks_and_drops_only_level_257() {
    for depth in [255, 256] {
        let source = nested_layers(depth, ".after{color:red}");
        let report = parse_sheet(&source);
        assert!(
            report.is_clean(),
            "depth {depth}: {:?}",
            report.diagnostics()
        );
        assert_eq!(report.syntax().rules().len(), 2);
        let CssRule::LayerBlock(layer) = &report.syntax().rules()[0] else {
            panic!("expected outer layer chain");
        };
        let mut layer = layer;
        let mut retained_depth = 1;
        while let [CssRule::LayerBlock(nested)] = layer.rules() {
            layer = nested;
            retained_depth += 1;
        }
        assert_eq!(retained_depth, depth);
    }

    let source = nested_layers(257, ".after{color:red}");
    let report = parse_sheet(&source);
    assert_eq!(
        report.syntax().rules().len(),
        2,
        "outer layer and later sibling survive"
    );
    assert!(matches!(report.syntax().rules()[1], CssRule::Style(_)));
    let (detail, span) = nesting_detail(&report);
    assert_eq!(detail.limit(), 256);
    assert_eq!(
        detail.enclosing_production().as_str(),
        "baseline.rule.layer-block"
    );
    let failed_start = "@layer{".len() * 256;
    assert_eq!(span.start().byte_offset().value(), failed_start);
    assert_eq!(
        span.end().byte_offset().value(),
        failed_start + "@layer{}".len()
    );
    assert_eq!(
        report.diagnostics()[0]
            .error()
            .position()
            .byte_offset()
            .value(),
        failed_start + "@layer".len(),
    );
}

#[test]
fn structural_recovery_nesting_limit_at_eof_spans_remaining_bounded_unit() {
    let source = "@layer{".repeat(257);
    let report = parse_sheet(&source);
    let (detail, span) = nesting_detail(&report);
    assert_eq!(
        detail.enclosing_production().as_str(),
        "baseline.rule.layer-block"
    );
    assert_eq!(span.start().byte_offset().value(), "@layer{".len() * 256);
    assert_eq!(span.end().byte_offset().value(), source.len());
}

#[test]
fn structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks() {
    for (opener, closer) in [("f(", ")"), ("(", ")"), ("[", "]"), ("{", "}")] {
        for depth in [255, 256] {
            let source = component_value(depth, opener, closer);
            let report = parse_sheet(&source);
            assert!(
                report.is_clean(),
                "{opener} at depth {depth}: {:?}",
                report.diagnostics()
            );
            let CssRule::Style(target) = &report.syntax().rules()[0] else {
                panic!("expected target style rule");
            };
            assert_eq!(target.declarations().len(), 2);
        }

        let source = component_value(257, opener, closer);
        let report = parse_sheet(&source);
        assert_eq!(report.syntax().rules().len(), 2);
        let CssRule::Style(target) = &report.syntax().rules()[0] else {
            panic!("expected retained target style rule");
        };
        assert_eq!(
            target.declarations().len(),
            1,
            "only the excessive declaration is dropped"
        );
        let (detail, span) = nesting_detail(&report);
        assert_eq!(detail.limit(), 256);
        assert_eq!(detail.enclosing_production().as_str(), "css.declaration");
        let declaration_start = source.find("--x:").expect("generated declaration");
        let declaration_end = source
            .find(";color")
            .expect("generated declaration boundary")
            + 1;
        assert_eq!(span.start().byte_offset().value(), declaration_start);
        assert_eq!(span.end().byte_offset().value(), declaration_end);
    }
}

#[test]
fn structural_recovery_drops_only_an_excessively_nested_descriptor() {
    let excessive = format!("src:{}x{};", "f(".repeat(256), ")".repeat(256));
    let source = format!(
        "@font-face{{font-family:Demo;{excessive}src:url(\"demo.woff2\");}}.after{{color:red}}"
    );

    let report = parse_sheet(&source);
    assert!(matches!(
        report.syntax().rules(),
        [CssRule::FontFace(_), CssRule::Style(_)]
    ));
    let (detail, span) = nesting_detail(&report);
    assert_eq!(detail.enclosing_production().as_str(), "css.descriptor");
    let start = source.find(&excessive).expect("generated descriptor");
    assert_eq!(span.start().byte_offset().value(), start);
    assert_eq!(span.end().byte_offset().value(), start + excessive.len());
}

#[test]
fn structural_recovery_reports_non_bmp_coordinates_and_keeps_later_siblings() {
    let prefix = "/* 🦊\n */";
    let source = format!("{prefix}{}", nested_layers(257, ".after{color:red}"));
    let report = parse_sheet(&source);
    let (_, span) = nesting_detail(&report);
    let failed_start = prefix.len() + "@layer{".len() * 256;
    assert_eq!(span.start().byte_offset().value(), failed_start);
    assert_eq!(span.start().line().value(), 1);
    assert_eq!(
        span.start().column().value(),
        3 + "@layer{".len() as u32 * 256
    );
    assert!(matches!(
        report.syntax().rules().last(),
        Some(CssRule::Style(_))
    ));
}

#[test]
fn malformed_at_rule_recovery_checks_balanced_component_depth() {
    for depth in [255, 256] {
        let source = malformed_block("@unknown{", depth, ".after{color:red}");
        let report = parse_sheet(&source);
        assert_eq!(report.diagnostics().len(), 1, "depth {depth}: {report:?}");
        assert_eq!(
            report.diagnostics()[0].action(),
            CssRecoveryAction::DropAtRule
        );
        assert!(matches!(report.syntax().rules(), [CssRule::Style(_)]));
    }

    let source = malformed_block("@unknown{", 257, ".after{color:red}");
    let unit_end = source.find(".after").expect("generated sibling boundary");
    let report = parse_sheet(&source);
    assert!(matches!(report.syntax().rules(), [CssRule::Style(_)]));
    let (detail, span) = nesting_detail(&report);
    assert_eq!(detail.enclosing_production().as_str(), "css.at-rule");
    assert_eq!(span.start().byte_offset().value(), 0);
    assert_eq!(span.end().byte_offset().value(), unit_end);
    assert_eq!(
        report.diagnostics()[0]
            .error()
            .position()
            .byte_offset()
            .value(),
        "@unknown{".len() + "f(".len() * 255,
    );
}

#[test]
fn nested_malformed_qualified_rule_recovery_checks_balanced_component_depth() {
    for depth in [255, 256] {
        let nested = malformed_block("???{", depth - 1, ".inside{color:blue}");
        let source = format!("@media screen{{{nested}}}.after{{color:red}}");
        let report = parse_sheet(&source);
        assert_eq!(report.diagnostics().len(), 1, "depth {depth}: {report:?}");
        assert_eq!(
            report.diagnostics()[0].action(),
            CssRecoveryAction::DropQualifiedRule
        );
        let CssRule::Media(media) = &report.syntax().rules()[0] else {
            panic!("expected retained media parent");
        };
        assert!(matches!(media.rules(), [CssRule::Style(_)]));
        assert!(matches!(
            report.syntax().rules().last(),
            Some(CssRule::Style(_))
        ));
    }

    let nested = malformed_block("???{", 256, ".inside{color:blue}");
    let source = format!("@media screen{{{nested}}}.after{{color:red}}");
    let unit_start = source.find("???").expect("generated failed unit");
    let unit_end = source.find(".inside").expect("generated nested sibling");
    let report = parse_sheet(&source);
    let CssRule::Media(media) = &report.syntax().rules()[0] else {
        panic!("expected retained media parent");
    };
    assert!(matches!(media.rules(), [CssRule::Style(_)]));
    assert!(matches!(
        report.syntax().rules().last(),
        Some(CssRule::Style(_))
    ));
    let (detail, span) = nesting_detail(&report);
    assert_eq!(detail.enclosing_production().as_str(), "css.qualified-rule");
    assert_eq!(span.start().byte_offset().value(), unit_start);
    assert_eq!(span.end().byte_offset().value(), unit_end);
    assert_eq!(
        report.diagnostics()[0]
            .error()
            .position()
            .byte_offset()
            .value(),
        unit_start + "???{".len() + "f(".len() * 254,
    );
}

#[test]
fn structural_preflight_ignores_delimiters_in_comments_strings_and_escapes() {
    let comment_delimiters = "({[".repeat(1024);
    let string_delimiters = "})]".repeat(1024);
    let source = format!(
        "/*{comment_delimiters}*/.target{{--x:\"{string_delimiters}\\\"tail\";--y:ident\\(\\[\\{{;color:blue}}.after{{color:red}}"
    );

    let report = parse_sheet(&source);
    assert!(report.is_clean(), "{report:?}");
    assert!(matches!(
        report.syntax().rules(),
        [CssRule::Style(_), CssRule::Style(_)]
    ));
}

#[test]
fn structural_preflight_accepts_256_mixed_scope_and_style_blocks() {
    let source = format!(
        "{} .x{{color:red}} {}",
        "@scope{".repeat(255),
        "}".repeat(255)
    );
    let report = parse_sheet(&source);
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::Scope(scope)] = report.syntax().rules() else {
        panic!("expected outer scope chain");
    };
    let mut scope = scope;
    let mut scope_depth = 1;
    while let [CssScopedRule::Scope(nested)] = scope.rules().rules() {
        scope = nested;
        scope_depth += 1;
    }
    assert_eq!(scope_depth, 255);
    assert!(matches!(scope.rules().rules(), [CssScopedRule::Style(_)]));
}

#[test]
fn structural_preflight_accepts_256_nested_style_blocks_without_losing_declarations() {
    let source = format!(
        "{}color:red{}}}.after{{color:blue}}",
        ".x{".repeat(256),
        "}".repeat(255),
    );
    let report = parse_sheet(&source);
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().rules().len(), 2);
    assert!(
        report
            .syntax()
            .rules()
            .iter()
            .all(|rule| matches!(rule, CssRule::Style(rule) if rule.declarations().len() == 1))
    );
}

#[test]
fn structural_recovery_finalizes_diagnostics_by_responsible_offset_with_stable_ties() {
    let source = "@keyframes fade { from { mystery: 1; } } .after { color: red; }";
    let report = parse_sheet(source);
    assert_eq!(report.diagnostics().len(), 3);
    let offsets: Vec<_> = report
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.error().position().byte_offset().value())
        .collect();
    assert!(offsets.windows(2).all(|pair| pair[0] <= pair[1]));
    assert_eq!(
        report.diagnostics()[0].action(),
        CssRecoveryAction::DropDeclaration
    );
    assert_eq!(
        report.diagnostics()[1].action(),
        CssRecoveryAction::DropKeyframeBlock
    );
    assert_eq!(
        report.diagnostics()[2].action(),
        CssRecoveryAction::DropAtRule
    );
}

#[test]
fn structural_recovery_never_unwinds_on_bounded_adversarial_text() {
    let deep = component_value(1024, "f(", ")");
    let repeated = format!("{}{}", "@bad{};".repeat(256), ".after{color:red}");
    let cases = [
        ("", false),
        (";;;;;}}}}\0\u{fffd}", false),
        ("🦊💥\n@unknown fn({a;b}); .after{color:red}", false),
        (deep.as_str(), true),
        (repeated.as_str(), true),
    ];

    for (source, expects_later_sibling) in cases {
        let result = catch_unwind(AssertUnwindSafe(|| parse_sheet(source)));
        let report = result.unwrap_or_else(|_| panic!("ordinary input unwound: {source:?}"));
        if source == deep {
            let (detail, span) = nesting_detail(&report);
            assert_eq!(detail.enclosing_production().as_str(), "css.declaration");
            let declaration_start = source.find("--x:").expect("generated declaration");
            let declaration_end = source
                .find(";color")
                .expect("generated declaration boundary")
                + 1;
            assert_eq!(span.start().byte_offset().value(), declaration_start);
            assert_eq!(span.end().byte_offset().value(), declaration_end);
        }
        if expects_later_sibling {
            assert!(
                matches!(report.syntax().rules().last(), Some(CssRule::Style(_))),
                "later sibling was lost for {source:?}: {report:?}"
            );
        }
    }
}
