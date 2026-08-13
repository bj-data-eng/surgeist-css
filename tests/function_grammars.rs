use surgeist_css::{
    CssErrorCode, CssKnownPropertyValueRef, CssLength, CssRecoveryAction, CssTransform,
    CssTransformAngle, CssTransformFunctionKind, CssTransformFunctionValue,
    CssTransformFunctionValueList, CssTransformLength, CssTransformLengthPercentage,
    CssTransformNonNegativeLength, CssTransformNumber, CssTransformPercentage,
    CssTransformPerspective, CssTransformPropertyValue, CssTransformScaleComponent,
    CssTransformValue, parse_style_attribute,
};

fn parsed_transform_property(value: &str) -> CssTransformPropertyValue {
    let report = parse_style_attribute(&format!("transform: {value}"));
    assert!(
        report.is_clean(),
        "expected `{value}` to parse cleanly, got {:?}",
        report.diagnostics()
    );
    let declaration = report.syntax()[0]
        .known()
        .expect("known transform declaration");
    let CssKnownPropertyValueRef::Transform(value) = declaration
        .property_value()
        .expect("ordinary transform value")
    else {
        panic!("expected transform property value");
    };
    value.clone()
}

fn assert_transform_rejected(value: &str) {
    let report = parse_style_attribute(&format!("transform: {value}"));
    assert!(report.syntax().is_empty(), "retained invalid `{value}`");
    assert_eq!(
        report.diagnostics().len(),
        1,
        "expected one diagnostic for `{value}`"
    );
}

fn assert_function_sequence(value: &str, expected: &[(CssTransformFunctionKind, &str)]) {
    let property = parsed_transform_property(value);
    let CssTransform::Functions(functions) = property
        .i01_subset()
        .expect("transform compatibility projection")
    else {
        panic!("expected transform function list");
    };
    let actual = functions
        .functions()
        .iter()
        .map(|function| (function.kind(), function.arguments().as_css()))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);

    let CssTransformValue::Functions(current) = property.current() else {
        panic!("expected current transform function list");
    };
    let current_kinds = current
        .functions()
        .iter()
        .map(CssTransformFunctionValue::kind)
        .collect::<Vec<_>>();
    let expected_kinds = expected.iter().map(|(kind, _)| *kind).collect::<Vec<_>>();
    assert_eq!(current_kinds, expected_kinds);
}

#[test]
fn every_selected_two_dimensional_transform_function_preserves_authored_order() {
    assert_function_sequence(
        concat!(
            "matrix(1, 0, 0, 1, 10, 20) ",
            "translate(1px, 2%) translateX(calc(1px + 2%)) translateY(3em) ",
            "scale(1.5, calc(1 + 0.5)) scaleX(calc(1 + 0.5)) scaleY(.75) ",
            "rotate(calc(1turn - 90deg)) skew(10deg, 0) skewX(.25turn) skewY(0)"
        ),
        &[
            (CssTransformFunctionKind::Matrix, "1, 0, 0, 1, 10, 20"),
            (CssTransformFunctionKind::Translate, "1px, 2%"),
            (CssTransformFunctionKind::TranslateX, "calc(1px + 2%)"),
            (CssTransformFunctionKind::TranslateY, "3em"),
            (CssTransformFunctionKind::Scale, "1.5, calc(1 + 0.5)"),
            (CssTransformFunctionKind::ScaleX, "calc(1 + 0.5)"),
            (CssTransformFunctionKind::ScaleY, "0.75"),
            (CssTransformFunctionKind::Rotate, "calc(1turn - 90deg)"),
            (CssTransformFunctionKind::Skew, "10deg, 0"),
            (CssTransformFunctionKind::SkewX, "0.25turn"),
            (CssTransformFunctionKind::SkewY, "0"),
        ],
    );
}

#[test]
fn transform_matrix3d_exposes_sixteen_finite_components() {
    assert_function_sequence(
        "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 10, 20, 30, 1)",
        &[(
            (CssTransformFunctionKind::Matrix3d),
            "1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 10, 20, 30, 1",
        )],
    );

    let property =
        parsed_transform_property("matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 10, 20, 30, 1)");
    let CssTransformValue::Functions(functions) = property.current() else {
        panic!("expected current transform function list");
    };
    let CssTransformFunctionValue::Matrix3d(matrix) = &functions.functions()[0] else {
        panic!("expected typed matrix3d");
    };
    assert!(matches!(
        matrix.components()[12],
        CssTransformNumber::Literal(value) if value.value() == 10.0
    ));
    assert!(matches!(
        matrix.components()[15],
        CssTransformNumber::Literal(value) if value.value() == 1.0
    ));
}

#[test]
fn transform_perspective_accepts_none_and_zero_and_rejects_invalid_dimensions() {
    assert_function_sequence(
        "perspective(none) perspective(0) perspective(12px)",
        &[
            (CssTransformFunctionKind::Perspective, "none"),
            (CssTransformFunctionKind::Perspective, "0"),
            (CssTransformFunctionKind::Perspective, "12px"),
        ],
    );

    for value in [
        "perspective(-1px)",
        "perspective(10%)",
        "perspective(auto)",
        "perspective()",
        "perspective(1px, 2px)",
    ] {
        assert_transform_rejected(value);
    }

    let property = parsed_transform_property("perspective(none) perspective(0)");
    let CssTransformValue::Functions(functions) = property.current() else {
        panic!("expected current transform function list");
    };
    assert!(matches!(
        functions.functions()[0],
        CssTransformFunctionValue::Perspective(CssTransformPerspective::None)
    ));
    assert!(matches!(
        &functions.functions()[1],
        CssTransformFunctionValue::Perspective(CssTransformPerspective::Length(length))
            if matches!(length.value(), CssLength::Zero)
    ));
}

#[test]
fn transform_three_dimensional_rotations_are_typed() {
    assert_function_sequence(
        "rotate3d(1, 0, -1, 45deg) rotateX(10deg) rotateY(0) rotateZ(calc(1turn / 2))",
        &[
            (CssTransformFunctionKind::Rotate3d, "1, 0, -1, 45deg"),
            (CssTransformFunctionKind::RotateX, "10deg"),
            (CssTransformFunctionKind::RotateY, "0"),
            (CssTransformFunctionKind::RotateZ, "calc(1turn / 2)"),
        ],
    );

    let property = parsed_transform_property("rotate3d(1, 0, -1, 45deg) rotateZ(calc(1turn / 2))");
    let CssTransformValue::Functions(functions) = property.current() else {
        panic!("expected current transform function list");
    };
    let CssTransformFunctionValue::Rotate3d(rotation) = &functions.functions()[0] else {
        panic!("expected typed rotate3d");
    };
    assert!(matches!(rotation.z(), CssTransformNumber::Literal(value) if value.value() == -1.0));
    assert!(matches!(rotation.angle(), CssTransformAngle::Literal(value) if value.value() == 45.0));
    assert!(matches!(
        functions.functions()[1],
        CssTransformFunctionValue::RotateZ(CssTransformAngle::Calculation(_))
    ));
}

#[test]
fn transform_three_dimensional_scales_preserve_number_and_percentage_operands() {
    assert_function_sequence(
        "scale3d(1, 50%, calc(1 + .5)) scaleZ(2) scaleZ(125%)",
        &[
            (CssTransformFunctionKind::Scale3d, "1, 50%, calc(1 + 0.5)"),
            (CssTransformFunctionKind::ScaleZ, "2"),
            (CssTransformFunctionKind::ScaleZ, "125%"),
        ],
    );

    let property =
        parsed_transform_property("scale3d(1, 50%, calc(1 + .5)) scaleZ(2) scaleZ(125%)");
    let CssTransformValue::Functions(functions) = property.current() else {
        panic!("expected current transform function list");
    };
    let CssTransformFunctionValue::Scale3d(scale) = &functions.functions()[0] else {
        panic!("expected typed scale3d");
    };
    assert!(matches!(
        scale.x(),
        CssTransformScaleComponent::Number(CssTransformNumber::Literal(value))
            if value.value() == 1.0
    ));
    assert!(matches!(
        scale.y(),
        CssTransformScaleComponent::Percentage(CssTransformPercentage::Literal(value))
            if value.value() == 50.0
    ));
    assert!(matches!(
        scale.z(),
        CssTransformScaleComponent::Number(CssTransformNumber::Calculation(_))
    ));
    assert!(matches!(
        functions.functions()[1],
        CssTransformFunctionValue::ScaleZ(CssTransformScaleComponent::Number(
            CssTransformNumber::Literal(value)
        )) if value.value() == 2.0
    ));
    assert!(matches!(
        functions.functions()[2],
        CssTransformFunctionValue::ScaleZ(CssTransformScaleComponent::Percentage(
            CssTransformPercentage::Literal(value)
        )) if value.value() == 125.0
    ));
}

#[test]
fn transform_three_dimensional_translations_keep_z_length_only() {
    assert_function_sequence(
        "translate3d(10%, calc(2px + 3%), 4em) translateZ(calc(1px + 2em))",
        &[
            (
                CssTransformFunctionKind::Translate3d,
                "10%, calc(2px + 3%), 4em",
            ),
            (CssTransformFunctionKind::TranslateZ, "calc(1px + 2em)"),
        ],
    );

    for value in ["translate3d(1px, 2px, 3%)", "translateZ(10%)"] {
        assert_transform_rejected(value);
    }

    let property = parsed_transform_property("translate3d(10%, calc(2px + 3%), 4em)");
    let CssTransformValue::Functions(functions) = property.current() else {
        panic!("expected current transform function list");
    };
    let CssTransformFunctionValue::Translate3d(translation) = &functions.functions()[0] else {
        panic!("expected typed translate3d");
    };
    assert!(matches!(translation.x().value(), CssLength::Percent(value) if value.value() == 10.0));
    assert!(matches!(translation.y().value(), CssLength::Calc(_)));
    assert!(matches!(translation.z().value(), CssLength::Dimension(value) if value.value() == 4.0));
}

#[test]
fn transform_functions_require_exact_commas_and_arities() {
    for value in [
        "matrix(1 0 0 1 10 20)",
        "matrix(1, 0, 0, 1, 10)",
        "matrix(1, 0, 0, 1, 10, 20, 30)",
        "translate(1px 2px)",
        "translateX(1px, 2px)",
        "scale(1 2)",
        "scale(1, 50%)",
        "scaleX()",
        "scaleY(50%)",
        "skew(10deg 20deg)",
        "rotate(10deg, 20deg)",
        "matrix3d(1 0 0 0 0 1 0 0 0 0 1 0 10 20 30 1)",
        "rotate3d(1 0 0 45deg)",
        "rotate3d(1, 0, 45deg)",
        "scale3d(1 2 3)",
        "scale3d(1, 2)",
        "translate3d(1px 2px 3px)",
        "translate3d(1px, 2px)",
    ] {
        assert_transform_rejected(value);
    }
}

#[test]
fn transform_function_lists_reject_empty_unknown_and_trailing_mutations() {
    for value in [
        "matrix()",
        "unknown(1)",
        "translateX(1px) trailing",
        "translateX(1px), rotate(1deg)",
    ] {
        assert_transform_rejected(value);
    }

    assert_function_sequence(
        "translateX(1px) rotate(2deg) scaleY(3)",
        &[
            (CssTransformFunctionKind::TranslateX, "1px"),
            (CssTransformFunctionKind::Rotate, "2deg"),
            (CssTransformFunctionKind::ScaleY, "3"),
        ],
    );
}

#[test]
fn transform_checked_scalars_and_lists_reject_unrepresentable_states() {
    assert!(CssTransformLengthPercentage::try_new(CssLength::Auto).is_none());
    assert!(
        CssTransformLength::try_new(CssLength::try_percent(10.0).expect("finite percentage"))
            .is_none()
    );
    assert!(
        CssTransformNonNegativeLength::try_new(
            CssLength::try_px(-1.0).expect("finite negative length"),
        )
        .is_none()
    );
    assert!(CssTransformFunctionValueList::try_new(Vec::new()).is_none());
}

#[test]
fn transform_calculations_preserve_the_exact_depth_boundary() {
    let source = format!(
        "transform: rotate({}1deg{}); color: red",
        "calc(".repeat(255),
        ")".repeat(255),
    );
    let report = parse_style_attribute(&source);
    assert!(report.is_clean(), "depth 255: {:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 2);

    for depth in [256_usize, 257] {
        let source = format!(
            "transform: rotate({}1deg{}); color: red",
            "calc(".repeat(depth),
            ")".repeat(depth),
        );
        let first_over_limit = source
            .match_indices("calc(")
            .nth(255)
            .expect("256th authored nested calculation")
            .0;
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "depth {depth}");
        let [diagnostic] = report.diagnostics() else {
            panic!("depth {depth}: over-limit transform must produce one diagnostic");
        };
        assert_eq!(diagnostic.error().code(), CssErrorCode::NestingLimit);
        assert_eq!(diagnostic.action(), CssRecoveryAction::StopAtNestingLimit);
        assert_eq!(
            diagnostic.error().position().byte_offset().value(),
            first_over_limit,
            "depth {depth}",
        );

        #[cfg(feature = "app-strict")]
        {
            let failure = surgeist_css::validate_style_attribute(&source)
                .expect_err("strict validation rejects over-limit transform calculations");
            assert_eq!(failure.diagnostics(), report.diagnostics());
        }
    }
}
