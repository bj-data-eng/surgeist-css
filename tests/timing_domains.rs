use surgeist_css::{
    CssAnimationDirection, CssAnimationFillMode, CssAnimationIterationNumber,
    CssAnimationIterationValue, CssAnimationIterationValueList, CssAnimationName,
    CssAnimationPlayState, CssCalculationExpressionRef, CssCalculationType, CssDelay, CssDelayList,
    CssDelayLiteral, CssDuration, CssDurationList, CssDurationLiteral, CssEasing, CssErrorCode,
    CssKnownPropertyValueRef, CssRecoveryAction, CssTimeUnit, CssTransitionProperty,
    parse_style_attribute,
};

#[test]
fn duration_and_delay_literals_enforce_distinct_finite_sign_domains() {
    let duration = CssDurationLiteral::try_new(1.5, CssTimeUnit::Seconds)
        .expect("finite non-negative duration");
    assert_eq!(duration.value(), 1.5);
    assert_eq!(duration.unit(), CssTimeUnit::Seconds);
    assert!(CssDurationLiteral::try_new(-0.25, CssTimeUnit::Seconds).is_none());

    let delay =
        CssDelayLiteral::try_new(-250.0, CssTimeUnit::Milliseconds).expect("finite signed delay");
    assert_eq!(delay.value(), -250.0);
    assert_eq!(delay.unit(), CssTimeUnit::Milliseconds);

    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(CssDurationLiteral::try_new(value, CssTimeUnit::Seconds).is_none());
        assert!(CssDelayLiteral::try_new(value, CssTimeUnit::Seconds).is_none());
    }

    assert!(CssDurationList::try_new(Vec::new()).is_none());
    assert!(CssDelayList::try_new(Vec::new()).is_none());
    assert_eq!(
        CssDurationList::try_new(vec![CssDuration::Literal(duration)])
            .expect("non-empty duration list")
            .values()
            .len(),
        1,
    );
    assert_eq!(
        CssDelayList::try_new(vec![CssDelay::Literal(delay)])
            .expect("non-empty delay list")
            .values()
            .len(),
        1,
    );
}

#[test]
fn iteration_values_check_literals_and_keep_infinite_and_calculation_branches() {
    assert!(CssAnimationIterationNumber::try_new(-0.25).is_none());
    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(CssAnimationIterationNumber::try_new(value).is_none());
    }
    let number = CssAnimationIterationNumber::try_new(2.5).expect("finite iteration number");
    assert_eq!(number.value(), 2.5);
    assert!(CssAnimationIterationValueList::try_new(Vec::new()).is_none());
    let values = CssAnimationIterationValueList::try_new(vec![
        CssAnimationIterationValue::Infinite,
        CssAnimationIterationValue::Number(number),
    ])
    .expect("non-empty iteration list");
    assert_eq!(values.values().len(), 2);
}

#[test]
fn timing_longhands_expose_exact_current_values_and_lossy_i01_boundaries() {
    let report = parse_style_attribute(concat!(
        "transition-duration: 1s, calc((2ms + 3ms) * 2); ",
        "transition-delay: -1s, 2ms; ",
        "animation-duration: 3ms, calc(1s + 2s); ",
        "animation-delay: calc(-1s + 2s), -4ms; ",
        "animation-iteration-count: infinite, 2.5, calc((1 + 2) * 3)"
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 5);

    let CssKnownPropertyValueRef::TransitionDuration(value) = report.syntax()[0]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected transition-duration wrapper");
    };
    assert!(matches!(
        value.durations().values()[0],
        CssDuration::Literal(literal)
            if literal.value() == 1.0 && literal.unit() == CssTimeUnit::Seconds
    ));
    let CssDuration::Calculation(calculation) = &value.durations().values()[1] else {
        panic!("expected duration calculation");
    };
    assert_eq!(calculation.result_type(), CssCalculationType::Time);
    assert!(matches!(
        calculation.expression(),
        CssCalculationExpressionRef::Product(_)
    ));
    assert!(value.i01_subset().is_none());

    let CssKnownPropertyValueRef::TransitionDelay(value) = report.syntax()[1]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected transition-delay wrapper");
    };
    assert!(matches!(
        value.delays().values()[0],
        CssDelay::Literal(literal)
            if literal.value() == -1.0 && literal.unit() == CssTimeUnit::Seconds
    ));
    assert!(matches!(value.delays().values()[1], CssDelay::Literal(_)));
    assert!(value.i01_subset().is_none());

    let CssKnownPropertyValueRef::AnimationDuration(value) = report.syntax()[2]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected animation-duration wrapper");
    };
    assert_eq!(value.durations().values().len(), 2);
    assert!(matches!(
        value.durations().values()[1],
        CssDuration::Calculation(_)
    ));
    assert!(value.i01_subset().is_none());

    let CssKnownPropertyValueRef::AnimationDelay(value) = report.syntax()[3]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected animation-delay wrapper");
    };
    assert!(matches!(
        value.delays().values()[0],
        CssDelay::Calculation(_)
    ));
    assert!(matches!(
        value.delays().values()[1],
        CssDelay::Literal(literal) if literal.value() == -4.0
    ));
    assert!(value.i01_subset().is_none());

    let CssKnownPropertyValueRef::AnimationIterationCount(value) = report.syntax()[4]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected animation-iteration-count wrapper");
    };
    let values = value.iteration_counts().values();
    assert!(matches!(values[0], CssAnimationIterationValue::Infinite));
    assert!(matches!(
        values[1],
        CssAnimationIterationValue::Number(number) if number.value() == 2.5
    ));
    assert!(matches!(
        values[2],
        CssAnimationIterationValue::Calculation(_)
    ));
    assert!(value.i01_subset().is_none());
}

#[test]
fn positive_i01_timing_inputs_keep_exact_compatibility_debug_observables() {
    let report = parse_style_attribute(concat!(
        "transition-duration: 1s, 2ms; transition-delay: 3s; ",
        "animation-duration: 4ms; animation-delay: 5s; ",
        "animation-iteration-count: infinite, 2; ",
        "transition: opacity 1s ease 2ms; ",
        "animation: fade 3s linear 2 4ms reverse both paused"
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    let values: Vec<String> = report
        .syntax()
        .iter()
        .map(|declaration| {
            let value = declaration.known().unwrap().property_value().unwrap();
            match value {
                CssKnownPropertyValueRef::TransitionDuration(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                CssKnownPropertyValueRef::TransitionDelay(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                CssKnownPropertyValueRef::AnimationDuration(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                CssKnownPropertyValueRef::AnimationDelay(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                CssKnownPropertyValueRef::AnimationIterationCount(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                CssKnownPropertyValueRef::Transition(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                CssKnownPropertyValueRef::Animation(value) => {
                    format!("{:?}", value.i01_subset().unwrap())
                }
                _ => panic!("unexpected timing wrapper"),
            }
        })
        .collect();
    assert_eq!(
        values,
        [
            "CssTimeList { times: [CssTime { value: 1.0, unit: Seconds }, CssTime { value: 2.0, unit: Milliseconds }] }",
            "CssTimeList { times: [CssTime { value: 3.0, unit: Seconds }] }",
            "CssTimeList { times: [CssTime { value: 4.0, unit: Milliseconds }] }",
            "CssTimeList { times: [CssTime { value: 5.0, unit: Seconds }] }",
            "CssAnimationIterationCountList { counts: [Infinite, Number(CssAnimationIterationNumber { value: 2.0 })] }",
            "CssTransitionList { items: [CssTransition { property: Some(Custom(CssCustomIdent(\"opacity\"))), duration: Some(CssTime { value: 1.0, unit: Seconds }), delay: Some(CssTime { value: 2.0, unit: Milliseconds }), timing_function: Some(Ease) }] }",
            "CssAnimationList { items: [CssAnimation { name: Some(Custom(CssCustomIdent(\"fade\"))), duration: Some(CssTime { value: 3.0, unit: Seconds }), delay: Some(CssTime { value: 4.0, unit: Milliseconds }), timing_function: Some(Linear), iteration_count: Some(Number(CssAnimationIterationNumber { value: 2.0 })), direction: Some(Reverse), fill_mode: Some(Both), play_state: Some(Paused) }] }",
        ]
    );
}

#[test]
fn transition_shorthand_assigns_first_time_to_duration_and_second_to_signed_delay() {
    let report = parse_style_attribute(concat!(
        "transition: opacity 1s ease -2ms, transform calc((3ms + 1ms) * 2) ",
        "calc(-1s + 2s) linear"
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let CssKnownPropertyValueRef::Transition(value) = report.syntax()[0]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected transition wrapper");
    };
    let transitions = value.transitions().values();
    assert_eq!(transitions.len(), 2);
    assert!(matches!(
        transitions[0].property(),
        Some(CssTransitionProperty::Custom(property)) if property.as_str() == "opacity"
    ));
    assert!(matches!(
        transitions[0].duration(),
        Some(CssDuration::Literal(_))
    ));
    assert!(matches!(
        transitions[0].delay(),
        Some(CssDelay::Literal(literal)) if literal.value() == -2.0
    ));
    assert!(matches!(
        transitions[0].timing_function(),
        Some(CssEasing::Ease)
    ));
    assert!(matches!(
        transitions[1].duration(),
        Some(CssDuration::Calculation(_))
    ));
    assert!(matches!(
        transitions[1].delay(),
        Some(CssDelay::Calculation(_))
    ));
    assert!(value.i01_subset().is_none());
}

#[test]
fn animation_shorthand_exposes_all_eight_current_components() {
    let report = parse_style_attribute(
        "animation: fade calc((1s + 2s) * 2) ease calc(-1s + 2s) calc((1 + 2) * 3) reverse both paused",
    );
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let CssKnownPropertyValueRef::Animation(value) = report.syntax()[0]
        .known()
        .unwrap()
        .property_value()
        .unwrap()
    else {
        panic!("expected animation wrapper");
    };
    let animation = &value.animations().values()[0];
    assert!(
        matches!(animation.name(), Some(CssAnimationName::Custom(name)) if name.as_str() == "fade")
    );
    assert!(matches!(
        animation.duration(),
        Some(CssDuration::Calculation(_))
    ));
    assert!(matches!(animation.delay(), Some(CssDelay::Calculation(_))));
    assert!(matches!(animation.timing_function(), Some(CssEasing::Ease)));
    assert!(matches!(
        animation.iteration_count(),
        Some(CssAnimationIterationValue::Calculation(_))
    ));
    assert_eq!(animation.direction(), Some(CssAnimationDirection::Reverse));
    assert_eq!(animation.fill_mode(), Some(CssAnimationFillMode::Both));
    assert_eq!(animation.play_state(), Some(CssAnimationPlayState::Paused));
    assert!(value.i01_subset().is_none());
}

#[test]
fn invalid_timing_order_lists_types_and_trailing_tokens_recover_later_siblings() {
    for source in [
        "transition-duration: -1s; color: red",
        "animation-duration: -1ms; color: red",
        "transition: opacity -1s 2s; color: red",
        "animation: fade -1s 2s; color: red",
        "transition: opacity 1s 2s 3s; color: red",
        "animation: fade 1s 2s 3s; color: red",
        "transition-duration: 1s,; color: red",
        "animation-delay: -1s,,2s; color: red",
        "transition-duration: calc(1px + 2px); color: red",
        "animation-iteration-count: calc(1s + 2s); color: red",
        "transition-delay: 1s trailing; color: red",
    ] {
        let report = parse_style_attribute(source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        let [diagnostic] = report.diagnostics() else {
            panic!("{source}: expected exactly one recovered declaration");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue,
            "{source}"
        );
        assert_eq!(
            diagnostic.action(),
            CssRecoveryAction::DropDeclaration,
            "{source}"
        );
    }
}

#[test]
fn non_finite_timing_and_iteration_tokens_are_rejected_without_losing_siblings() {
    for source in [
        "transition-duration: 1e999s; color: red",
        "transition-delay: -1e999s; color: red",
        "animation-duration: calc(1e999s); color: red",
        "animation-delay: calc(-1e999s); color: red",
        "animation-iteration-count: 1e999; color: red",
        "animation-iteration-count: calc(1e999); color: red",
    ] {
        let report = parse_style_attribute(source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(report.diagnostics().len(), 1, "{source}");
        assert_eq!(
            report.diagnostics()[0].error().code(),
            CssErrorCode::InvalidPropertyValue,
            "{source}",
        );
    }
}

#[test]
fn repeated_timing_failures_and_calculation_depth_boundaries_are_panic_free() {
    let mut source = String::new();
    for _ in 0..128 {
        source.push_str("transition: -1s 2s;");
    }
    source.push_str("color: red");
    let report = parse_style_attribute(&source);
    assert_eq!(report.syntax().len(), 1);
    assert_eq!(report.diagnostics().len(), 128);

    for depth in [255_usize, 256] {
        let source = format!(
            "transition-duration: {}1s{}; color: red",
            "calc(".repeat(depth),
            ")".repeat(depth)
        );
        let report = parse_style_attribute(&source);
        assert!(
            report.is_clean(),
            "depth {depth}: {:?}",
            report.diagnostics()
        );
        assert_eq!(report.syntax().len(), 2);
    }

    let depth = 257_usize;
    let source = format!(
        "transition-duration: {}1s{}; color: red",
        "calc(".repeat(depth),
        ")".repeat(depth)
    );
    let report = parse_style_attribute(&source);
    assert_eq!(report.syntax().len(), 1);
    let [diagnostic] = report.diagnostics() else {
        panic!("over-limit timing calculation must produce one diagnostic");
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::NestingLimit);
    assert_eq!(diagnostic.action(), CssRecoveryAction::StopAtNestingLimit);
}
