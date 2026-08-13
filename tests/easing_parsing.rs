use surgeist_css::{
    CssErrorCode, CssKnownProperty, CssRecoveryAction, CssTokenKind, ErrorKind,
    parse_style_attribute,
};

struct InvalidEasingCase {
    source: &'static str,
    property: CssKnownProperty,
    position: usize,
    span_end: usize,
    encountered: Option<(CssTokenKind, &'static str)>,
}

fn assert_invalid_easing_is_dropped(case: &InvalidEasingCase) {
    let report = parse_style_attribute(case.source);
    let [declaration] = report.syntax().as_slice() else {
        panic!(
            "{}: invalid easing must be dropped while the valid sibling is retained; syntax={:?}",
            case.source,
            report.syntax(),
        );
    };
    assert_eq!(
        declaration
            .known()
            .expect("retained color declaration")
            .property(),
        CssKnownProperty::Color,
        "{}",
        case.source,
    );

    let [diagnostic] = report.diagnostics() else {
        panic!(
            "{}: expected one exact invalid-easing diagnostic; diagnostics={:?}",
            case.source,
            report.diagnostics(),
        );
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue,
        "{}",
        case.source,
    );
    assert_eq!(
        diagnostic.action(),
        CssRecoveryAction::DropDeclaration,
        "{}",
        case.source,
    );
    assert_eq!(
        diagnostic.error().position().byte_offset().value(),
        case.position
    );
    assert_eq!(diagnostic.span().start().byte_offset().value(), 0);
    assert_eq!(
        diagnostic.span().end().byte_offset().value(),
        case.span_end,
        "{}",
        case.source,
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("{}: expected typed property-value detail", case.source);
    };
    assert_eq!(detail.property(), case.property, "{}", case.source);
    assert_eq!(
        detail.expectation().as_str(),
        "a value accepted by the property's grammar",
        "{}",
        case.source,
    );
    match (detail.encountered(), case.encountered) {
        (Some(actual), Some((kind, authored))) => {
            assert_eq!(actual.kind(), kind, "{}", case.source);
            assert_eq!(actual.authored(), authored, "{}", case.source);
        }
        (None, None) => {}
        (actual, expected) => panic!(
            "{}: encountered token mismatch: actual={actual:?}, expected={expected:?}",
            case.source,
        ),
    }

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(case.source)
            .expect_err("strict validation must reject the recovered easing declaration");
        assert_eq!(
            failure.diagnostics(),
            report.diagnostics(),
            "{}",
            case.source
        );
    }
}

#[test]
fn cubic_bezier_rejects_x_coordinates_outside_the_closed_unit_interval() {
    for case in [
        InvalidEasingCase {
            source: "transition-timing-function: cubic-bezier(-0.01, 0, 0.5, 1); color: red",
            property: CssKnownProperty::TransitionTimingFunction,
            position: 28,
            span_end: 64,
            encountered: Some((CssTokenKind::Function, "cubic-bezier")),
        },
        InvalidEasingCase {
            source: "animation-timing-function: cubic-bezier(0.5, -20, 1.01, 20); color: red",
            property: CssKnownProperty::AnimationTimingFunction,
            position: 27,
            span_end: 63,
            encountered: Some((CssTokenKind::Function, "cubic-bezier")),
        },
        InvalidEasingCase {
            source: "transition: opacity 1s cubic-bezier(2, -20, 0, 20); color: red",
            property: CssKnownProperty::Transition,
            position: 25,
            span_end: 51,
            encountered: Some((CssTokenKind::Function, "cubic-bezier")),
        },
        InvalidEasingCase {
            source: "animation: fade 1s cubic-bezier(0, -20, -1, 20); color: red",
            property: CssKnownProperty::Animation,
            position: 19,
            span_end: 50,
            encountered: Some((CssTokenKind::Function, "cubic-bezier")),
        },
    ] {
        assert_invalid_easing_is_dropped(&case);
    }
}

#[test]
fn steps_jump_none_rejects_a_single_interval() {
    for case in [
        InvalidEasingCase {
            source: "transition-timing-function: steps(1, jump-none); color: red",
            property: CssKnownProperty::TransitionTimingFunction,
            position: 28,
            span_end: 47,
            encountered: Some((CssTokenKind::Function, "steps")),
        },
        InvalidEasingCase {
            source: "animation-timing-function: steps(1, JUMP-NONE); color: red",
            property: CssKnownProperty::AnimationTimingFunction,
            position: 27,
            span_end: 46,
            encountered: Some((CssTokenKind::Function, "steps")),
        },
        InvalidEasingCase {
            source: "transition: opacity 1s steps(1, jump-none); color: red",
            property: CssKnownProperty::Transition,
            position: 25,
            span_end: 44,
            encountered: Some((CssTokenKind::Function, "steps")),
        },
        InvalidEasingCase {
            source: "animation: fade 1s steps(1, jump-none); color: red",
            property: CssKnownProperty::Animation,
            position: 19,
            span_end: 38,
            encountered: Some((CssTokenKind::Function, "steps")),
        },
    ] {
        assert_invalid_easing_is_dropped(&case);
    }
}
