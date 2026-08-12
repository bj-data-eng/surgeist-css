use surgeist_css::{
    CssAngleCalculation, CssAngleUnit, CssCalculationExpressionRef, CssCalculationType,
    CssCalculationValueRef, CssErrorCode, CssFrequencyCalculation, CssFrequencyUnit,
    CssIntegerCalculation, CssKnownProperty, CssLengthCalculation, CssLengthUnit,
    CssNumberCalculation, CssPercentageCalculation, CssRecoveryAction, CssTimeCalculation,
    CssTimeUnit, CssTokenKind, ErrorKind, parse_style_attribute,
};

#[test]
fn number_calculation_literal_preserves_finite_authored_value() {
    let calculation =
        CssNumberCalculation::try_literal(-3.5).expect("finite authored number calculation");

    assert_eq!(calculation.result_type(), CssCalculationType::Number);
    match calculation.expression() {
        CssCalculationExpressionRef::Value(CssCalculationValueRef::Number(value)) => {
            assert_eq!(value.value(), -3.5);
        }
        _ => panic!("expected an authored number leaf"),
    }
}

#[test]
fn property_consumers_defer_typed_product_and_group_integration_with_exact_recovery() {
    for (value, authored, kind) in [
        ("calc(1px * 2)", "*", CssTokenKind::Delim),
        ("calc((1px + 2px))", "(", CssTokenKind::ParenthesisBlock),
    ] {
        let invalid = format!("width: {value};");
        let source = format!("{invalid} color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1);
        let [diagnostic] = report.diagnostics() else {
            panic!("staged property integration must recover exactly once");
        };
        assert_eq!(
            diagnostic.error().code(),
            CssErrorCode::InvalidPropertyValue
        );
        assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
        assert_eq!(diagnostic.span().start().byte_offset().value(), 0);
        assert_eq!(diagnostic.span().end().byte_offset().value(), invalid.len());
        match diagnostic.error().kind() {
            ErrorKind::InvalidPropertyValue(detail) => {
                assert_eq!(detail.property(), CssKnownProperty::Width);
                let encountered = detail.encountered().expect("responsible authored token");
                assert_eq!(encountered.kind(), kind);
                assert_eq!(encountered.authored(), authored);
            }
            _ => panic!("expected a structured width value error"),
        }
    }
}

#[test]
fn typed_calculation_roots_enforce_checked_literal_boundaries() {
    let integer_min = CssIntegerCalculation::literal(i32::MIN);
    let integer_max = CssIntegerCalculation::literal(i32::MAX);
    assert_eq!(integer_min.result_type(), CssCalculationType::Integer);
    assert_eq!(integer_max.result_type(), CssCalculationType::Integer);
    assert!(matches!(
        integer_min.expression(),
        CssCalculationExpressionRef::Value(CssCalculationValueRef::Integer(i32::MIN))
    ));
    assert!(matches!(
        integer_max.expression(),
        CssCalculationExpressionRef::Value(CssCalculationValueRef::Integer(i32::MAX))
    ));

    for value in [f32::MIN, -0.0, f32::MAX] {
        let number = CssNumberCalculation::try_literal(value).expect("finite number leaf");
        assert_eq!(number.result_type(), CssCalculationType::Number);
        assert!(matches!(
            number.expression(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Number(inner))
                if inner.value() == value
        ));
    }

    for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(CssNumberCalculation::try_literal(value).is_none());
        assert!(CssPercentageCalculation::try_literal(value).is_none());
        assert!(CssLengthCalculation::try_dimension(value, CssLengthUnit::Rem).is_none());
        assert!(CssLengthCalculation::try_percentage(value).is_none());
        assert!(CssAngleCalculation::try_literal(value, CssAngleUnit::Degrees).is_none());
        assert!(CssTimeCalculation::try_literal(value, CssTimeUnit::Seconds).is_none());
        assert!(CssFrequencyCalculation::try_literal(value, CssFrequencyUnit::Hertz).is_none());
    }

    for value in [f32::MIN, -0.0, f32::MAX] {
        let percentage =
            CssPercentageCalculation::try_literal(value).expect("finite percentage leaf");
        assert_eq!(percentage.result_type(), CssCalculationType::Percentage);
        assert!(matches!(
            percentage.expression(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Percentage(inner))
                if inner.value() == value
        ));

        let length = CssLengthCalculation::try_dimension(value, CssLengthUnit::Cqw)
            .expect("finite length leaf");
        assert_eq!(length.result_type(), CssCalculationType::Length);
        assert!(matches!(
            length.expression(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Length(inner))
                if inner.value() == value && inner.unit() == CssLengthUnit::Cqw
        ));

        let angle = CssAngleCalculation::try_literal(value, CssAngleUnit::Turns)
            .expect("finite angle leaf");
        assert_eq!(angle.result_type(), CssCalculationType::Angle);
        assert!(matches!(
            angle.expression(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Angle(inner))
                if inner.value() == value && inner.unit() == CssAngleUnit::Turns
        ));

        let time = CssTimeCalculation::try_literal(value, CssTimeUnit::Milliseconds)
            .expect("finite signed time leaf");
        assert_eq!(time.result_type(), CssCalculationType::Time);
        assert!(matches!(
            time.expression(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Time(inner))
                if inner.value() == value && inner.unit() == CssTimeUnit::Milliseconds
        ));

        let frequency = CssFrequencyCalculation::try_literal(value, CssFrequencyUnit::Kilohertz)
            .expect("finite frequency leaf");
        assert_eq!(frequency.result_type(), CssCalculationType::Frequency);
        assert!(matches!(
            frequency.expression(),
            CssCalculationExpressionRef::Value(CssCalculationValueRef::Frequency(inner))
                if inner.value() == value && inner.unit() == CssFrequencyUnit::Kilohertz
        ));
    }

    let percentage_length =
        CssLengthCalculation::try_percentage(-25.0).expect("finite signed percentage");
    assert_eq!(
        percentage_length.result_type(),
        CssCalculationType::Percentage
    );
    assert!(matches!(
        percentage_length.expression(),
        CssCalculationExpressionRef::Value(CssCalculationValueRef::Percentage(value))
            if value.value() == -25.0
    ));
}

#[test]
fn existing_calc_consumer_preserves_exact_depth_boundary_and_later_sibling() {
    for depth in [255_usize, 256] {
        let source = format!(
            "width: {}1px{}; color: red",
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
        "width: {}1px{}; color: red",
        "calc(".repeat(depth),
        ")".repeat(depth)
    );
    let first_over_limit = source
        .match_indices("calc(")
        .nth(256)
        .expect("257th authored calculation")
        .0;
    let report = parse_style_attribute(&source);
    assert_eq!(report.syntax().len(), 1);
    let [diagnostic] = report.diagnostics() else {
        panic!("over-limit calculation must produce one diagnostic");
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::NestingLimit);
    assert_eq!(diagnostic.action(), CssRecoveryAction::StopAtNestingLimit);
    assert_eq!(
        diagnostic.error().position().byte_offset().value(),
        first_over_limit
    );
}
