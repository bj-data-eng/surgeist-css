use surgeist_css::{
    CssKnownPropertyValueRef, CssTransform, CssTransformFunctionKind, parse_style_attribute,
};

fn parsed_transform(value: &str) -> CssTransform {
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
    value
        .i01_subset()
        .expect("transform compatibility projection")
        .clone()
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
    let CssTransform::Functions(functions) = parsed_transform(value) else {
        panic!("expected transform function list");
    };
    let actual = functions
        .functions()
        .iter()
        .map(|function| (function.kind(), function.arguments().as_css()))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn every_selected_two_dimensional_transform_function_preserves_authored_order() {
    assert_function_sequence(
        concat!(
            "matrix(1, 0, 0, 1, 10, 20) ",
            "translate(1px, 2%) translateX(calc(1px + 2%)) translateY(3em) ",
            "scale(1.5, 25%) scaleX(calc(1 + 0.5)) scaleY(75%) ",
            "rotate(calc(1turn - 90deg)) skew(10deg, 0) skewX(.25turn) skewY(0)"
        ),
        &[
            (CssTransformFunctionKind::Matrix, "1, 0, 0, 1, 10, 20"),
            (CssTransformFunctionKind::Translate, "1px, 2%"),
            (CssTransformFunctionKind::TranslateX, "calc(1px + 2%)"),
            (CssTransformFunctionKind::TranslateY, "3em"),
            (CssTransformFunctionKind::Scale, "1.5, 25%"),
            (CssTransformFunctionKind::ScaleX, "calc(1 + 0.5)"),
            (CssTransformFunctionKind::ScaleY, "75%"),
            (CssTransformFunctionKind::Rotate, "calc(1turn - 90deg)"),
            (CssTransformFunctionKind::Skew, "10deg, 0"),
            (CssTransformFunctionKind::SkewX, ".25turn"),
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
}

#[test]
fn transform_three_dimensional_scales_preserve_number_and_percentage_operands() {
    assert_function_sequence(
        "scale3d(1, 50%, calc(1 + .5)) scaleZ(125%)",
        &[
            (CssTransformFunctionKind::Scale3d, "1, 50%, calc(1 + .5)"),
            (CssTransformFunctionKind::ScaleZ, "125%"),
        ],
    );
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
        "scaleX()",
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
