use surgeist_css::{
    CssCalcLength, CssErrorCode, CssHorizontalPositionKeyword, CssKnownProperty,
    CssKnownPropertyValueRef, CssLength, CssLengthCalculation, CssLengthUnit, CssPositionComponent,
    CssPositionOffset, CssRecoveryAction, CssVerticalPositionKeyword, parse_style_attribute,
};

fn assert_generic_position_accepted(value: &str) {
    let source = format!("mask-position: {value}");
    let report = parse_style_attribute(&source);
    assert!(report.is_clean(), "{source}: {:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 1, "{source}");
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("known mask-position declaration")
            .property(),
        CssKnownProperty::MaskPosition,
        "{source}",
    );
}

fn assert_generic_position_rejected(value: &str) {
    let source = format!("mask-position: {value}; color: red");
    let report = parse_style_attribute(&source);
    assert_eq!(report.syntax().len(), 1, "{source}");
    assert_eq!(report.diagnostics().len(), 1, "{source}");
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("retained color declaration")
            .property(),
        CssKnownProperty::Color,
        "{source}",
    );

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(&source)
            .expect_err("strict validation rejects a recovered generic position");
        assert_eq!(failure.diagnostics(), report.diagnostics(), "{source}");
    }
}

#[test]
fn generic_position_accepts_every_one_two_and_four_component_branch() {
    for value in [
        "left",
        "top",
        "25%",
        "left top",
        "top left",
        "center bottom",
        "bottom center",
        "center center",
        "left 25%",
        "center 25%",
        "25% top",
        "25% center",
        "25% 75%",
        "left 10px top 20%",
        "bottom 20% right 10px",
        "left calc(10px + 5%) bottom calc(20px + 10%)",
    ] {
        assert_generic_position_accepted(value);
    }
}

#[test]
fn generic_position_rejects_axis_order_partial_pairs_and_duplicate_axes() {
    for value in [
        "left right",
        "50% left",
        "left top 10px",
        "left 10px top",
        "center 10px top 20px",
        "left 10px right 20px",
        "top 10px",
        "bottom calc(1px * 2)",
    ] {
        assert_generic_position_rejected(value);
    }
}

#[test]
fn deferred_background_position_preserves_three_component_legacy_projection() {
    let report = parse_style_attribute("background-position: left 10px top");
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let declaration = report.syntax()[0]
        .known()
        .expect("known background-position declaration");
    let CssKnownPropertyValueRef::BackgroundPosition(value) = declaration
        .property_value()
        .expect("ordinary background-position value")
    else {
        panic!("expected background-position value");
    };
    let positions = value
        .i01_subset()
        .expect("deferred background-position retains its I01 projection")
        .positions();
    assert_eq!(positions.len(), 1);
    assert!(matches!(
        positions[0].components(),
        [
            CssPositionComponent::Horizontal(CssHorizontalPositionKeyword::Left),
            CssPositionComponent::Length(CssLength::Px(length)),
            CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
        ] if length.value() == 10.0
    ));
}

#[test]
fn deferred_transform_origin_preserves_vertical_length_legacy_projection() {
    let report = parse_style_attribute("transform-origin: top 10px");
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let declaration = report.syntax()[0]
        .known()
        .expect("known transform-origin declaration");
    let CssKnownPropertyValueRef::TransformOrigin(value) = declaration
        .property_value()
        .expect("ordinary transform-origin value")
    else {
        panic!("expected transform-origin value");
    };
    let position = value
        .i01_subset()
        .expect("deferred transform-origin retains its I01 projection");
    assert!(matches!(
        position.components(),
        [
            CssPositionComponent::Vertical(CssVerticalPositionKeyword::Top),
            CssPositionComponent::Length(CssLength::Px(length)),
        ] if length.value() == 10.0
    ));
}

#[test]
fn position_offset_construction_accepts_only_authored_length_percentages() {
    for value in [
        CssLength::try_px(10.0).expect("finite px"),
        CssLength::try_dimension(-2.0, CssLengthUnit::Em).expect("finite dimension"),
        CssLength::try_percent(25.0).expect("finite percentage"),
        CssLength::Zero,
        CssLength::Calc(CssCalcLength::Typed(
            CssLengthCalculation::try_percentage(40.0).expect("finite calculation leaf"),
        )),
    ] {
        let offset = CssPositionOffset::try_new(value.clone()).expect("position-valid offset");
        assert_eq!(offset.value(), &value);
    }

    for value in [
        CssLength::Auto,
        CssLength::MinContent,
        CssLength::MaxContent,
        CssLength::FitContent,
        CssLength::Normal,
    ] {
        assert!(CssPositionOffset::try_new(value).is_none());
    }
}

#[test]
fn generic_position_typed_calculation_preserves_the_exact_depth_boundary() {
    for depth in [255_usize, 256] {
        let source = format!(
            "mask-position: left {}1px{}; color: red",
            "calc(".repeat(depth),
            ")".repeat(depth),
        );
        let report = parse_style_attribute(&source);
        assert!(
            report.is_clean(),
            "depth {depth}: {:?}",
            report.diagnostics(),
        );
        assert_eq!(report.syntax().len(), 2, "depth {depth}");
    }

    let depth = 257_usize;
    let source = format!(
        "mask-position: left {}1px{}; color: red",
        "calc(".repeat(depth),
        ")".repeat(depth),
    );
    let first_over_limit = source
        .match_indices("calc(")
        .nth(256)
        .expect("257th authored position calculation")
        .0;
    let report = parse_style_attribute(&source);
    assert_eq!(report.syntax().len(), 1);
    let [diagnostic] = report.diagnostics() else {
        panic!("over-limit position calculation must produce one diagnostic");
    };
    assert_eq!(diagnostic.error().code(), CssErrorCode::NestingLimit);
    assert_eq!(diagnostic.action(), CssRecoveryAction::StopAtNestingLimit);
    assert_eq!(
        diagnostic.error().position().byte_offset().value(),
        first_over_limit,
    );

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(&source)
            .expect_err("strict validation rejects over-limit position calculations");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}
