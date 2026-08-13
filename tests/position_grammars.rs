use surgeist_css::{
    CssBackgroundRepeat, CssBackgroundRepeatStyle, CssBackgroundSize, CssCalcLength, CssErrorCode,
    CssHorizontalPosition, CssHorizontalPositionKeyword, CssImageLayer, CssKnownProperty,
    CssKnownPropertyValueRef, CssLength, CssLengthCalculation, CssLengthUnit, CssMaskLayer,
    CssMaskList, CssPosition, CssPositionComponent, CssPositionOffset, CssRecoveryAction,
    CssTokenKind, CssUrl, CssVerticalPosition, CssVerticalPositionKeyword, ErrorKind,
    parse_style_attribute,
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

#[test]
fn background_and_mask_position_lists_expose_each_exact_layer() {
    let report = parse_style_attribute(concat!(
        "background-position: left 10px top, center bottom 25%; ",
        "mask-position: right 5% bottom 2px, calc((1px + 2%) * 3) top",
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    let CssKnownPropertyValueRef::BackgroundPosition(background) = report.syntax()[0]
        .known()
        .expect("known background position")
        .property_value()
        .expect("ordinary background position")
    else {
        panic!("expected background-position value");
    };
    let background_layers = background.positions().positions();
    assert_eq!(background_layers.len(), 2);
    assert!(matches!(
        background_layers[0].horizontal(),
        CssHorizontalPosition::LeftOffset(offset)
            if matches!(offset.value(), CssLength::Px(value) if value.value() == 10.0)
    ));
    assert!(matches!(
        background_layers[0].vertical(),
        CssVerticalPosition::Top
    ));
    assert!(matches!(
        background_layers[1].horizontal(),
        CssHorizontalPosition::Center
    ));
    assert!(matches!(
        background_layers[1].vertical(),
        CssVerticalPosition::BottomOffset(offset)
            if matches!(offset.value(), CssLength::Percent(value) if value.value() == 25.0)
    ));

    let CssKnownPropertyValueRef::MaskPosition(mask) = report.syntax()[1]
        .known()
        .expect("known mask position")
        .property_value()
        .expect("ordinary mask position")
    else {
        panic!("expected mask-position value");
    };
    let mask_layers = mask.positions().positions();
    assert_eq!(mask_layers.len(), 2);
    assert!(matches!(
        mask_layers[0].value().horizontal(),
        CssHorizontalPosition::RightOffset(offset)
            if matches!(offset.value(), CssLength::Percent(value) if value.value() == 5.0)
    ));
    assert!(matches!(
        mask_layers[0].value().vertical(),
        CssVerticalPosition::BottomOffset(offset)
            if matches!(offset.value(), CssLength::Px(value) if value.value() == 2.0)
    ));
    assert!(matches!(
        mask_layers[1].value().horizontal(),
        CssHorizontalPosition::Offset(offset)
            if matches!(offset.value(), CssLength::Calc(_))
    ));
    assert!(matches!(
        mask_layers[1].value().vertical(),
        CssVerticalPosition::Top
    ));
}

#[test]
fn background_accepts_three_components_that_mask_rejects() {
    for value in [
        "left 10px top",
        "left top 10px",
        "top left 10px",
        "top 10px left",
        "center bottom 20%",
        "right calc((1px + 2%) * 3) center",
    ] {
        let background = parse_style_attribute(&format!("background-position: {value}"));
        assert!(
            background.is_clean(),
            "background-position: {value}: {:?}",
            background.diagnostics(),
        );

        let mask_source = format!("mask-position: {value}; color: red");
        let mask = parse_style_attribute(&mask_source);
        assert_eq!(mask.syntax().len(), 1, "{mask_source}");
        assert_eq!(mask.diagnostics().len(), 1, "{mask_source}");
        assert_eq!(
            mask.syntax()[0].known().expect("retained color").property(),
            CssKnownProperty::Color,
            "{mask_source}",
        );

        #[cfg(feature = "app-strict")]
        {
            let failure = surgeist_css::validate_style_attribute(&mask_source)
                .expect_err("strict validation rejects mask-only three-component syntax");
            assert_eq!(failure.diagnostics(), mask.diagnostics(), "{mask_source}");
        }
    }
}

#[test]
fn mask_shorthand_rejects_a_three_component_position_and_recovers_at_the_declaration() {
    let source = "mask: url(mask.png) left 10px top / contain no-repeat; color: red";
    let report = parse_style_attribute(source);

    assert_eq!(report.syntax().len(), 1);
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("retained color declaration")
            .property(),
        CssKnownProperty::Color,
    );
    let [diagnostic] = report.diagnostics() else {
        panic!("three-component mask position must produce one diagnostic");
    };
    assert_eq!(
        diagnostic.error().code(),
        CssErrorCode::InvalidPropertyValue
    );
    assert_eq!(diagnostic.action(), CssRecoveryAction::DropDeclaration);
    let responsible_offset = source.find("left").expect("invalid position start");
    assert_eq!(
        diagnostic.error().position().byte_offset().value(),
        responsible_offset,
    );
    assert_eq!(diagnostic.error().position().line().value(), 0);
    assert_eq!(
        diagnostic.error().position().column().value() as usize,
        responsible_offset,
    );
    assert_eq!(diagnostic.span().start().byte_offset().value(), 0);
    assert_eq!(
        diagnostic.span().end().byte_offset().value(),
        source.find(';').expect("mask declaration semicolon") + 1,
    );
    let ErrorKind::InvalidPropertyValue(detail) = diagnostic.error().kind() else {
        panic!("expected structured mask property-value error");
    };
    assert_eq!(detail.property(), CssKnownProperty::Mask);
    let encountered = detail.encountered().expect("invalid position start token");
    assert_eq!(encountered.kind(), CssTokenKind::Ident);
    assert_eq!(encountered.authored(), "left");

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation rejects recovered mask shorthand");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn mask_shorthand_preserves_valid_image_position_size_and_repeat_components() {
    let source = "mask: url(mask.png) center / contain no-repeat";
    let report = parse_style_attribute(source);
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    let [declaration] = report.syntax().as_slice() else {
        panic!("valid mask shorthand must retain one declaration");
    };
    let known = declaration.known().expect("known mask declaration");
    assert_eq!(known.property(), CssKnownProperty::Mask);
    let CssKnownPropertyValueRef::Mask(value) = known
        .property_value()
        .expect("ordinary mask shorthand value")
    else {
        panic!("expected typed mask shorthand value");
    };
    assert_eq!(value.as_css(), "url(mask.png) center / contain no-repeat");

    let expected = CssMaskList::try_new(vec![
        CssMaskLayer::try_new(
            Some(CssImageLayer::Url(
                CssUrl::try_new("mask.png").expect("nonempty URL"),
            )),
            Some(
                CssPosition::try_new(vec![CssPositionComponent::Horizontal(
                    CssHorizontalPositionKeyword::Center,
                )])
                .expect("valid center position"),
            ),
            Some(CssBackgroundSize::Contain),
            Some(CssBackgroundRepeat::Axes {
                x: CssBackgroundRepeatStyle::NoRepeat,
                y: CssBackgroundRepeatStyle::NoRepeat,
            }),
        )
        .expect("nonempty mask layer"),
    ])
    .expect("nonempty mask list");
    assert_eq!(value.i01_subset(), Some(&expected));

    #[cfg(feature = "app-strict")]
    {
        let strict = surgeist_css::validate_style_attribute(source)
            .expect("strict validation accepts valid mask shorthand");
        assert_eq!(strict, report.syntax().clone());
    }
}

#[test]
fn layered_positions_reject_empty_items_slashes_and_trailing_components() {
    for (property, value) in [
        ("background-position", ""),
        ("background-position", "left,"),
        ("background-position", "left,,right"),
        ("background-position", "left / cover"),
        ("background-position", "left top 10px 20px"),
        ("mask-position", ""),
        ("mask-position", "left,"),
        ("mask-position", "left,,right"),
        ("mask-position", "left / cover"),
        ("mask-position", "left 10px top"),
    ] {
        let source = format!("{property}: {value}; color: red");
        let report = parse_style_attribute(&source);
        assert_eq!(report.syntax().len(), 1, "{source}");
        assert_eq!(report.diagnostics().len(), 1, "{source}");
        assert_eq!(
            report.syntax()[0]
                .known()
                .expect("retained color")
                .property(),
            CssKnownProperty::Color,
            "{source}",
        );
    }
}

#[test]
fn layered_position_failures_drop_each_declaration_and_continue() {
    let source = concat!(
        "background-position: left, left right, bottom; ",
        "mask-position: top 10px, center; ",
        "color: red",
    );
    let report = parse_style_attribute(source);
    assert_eq!(report.syntax().len(), 1);
    assert_eq!(report.diagnostics().len(), 2);
    assert_eq!(
        report.syntax()[0]
            .known()
            .expect("retained color")
            .property(),
        CssKnownProperty::Color,
    );
    assert!(report.diagnostics().iter().all(|diagnostic| {
        diagnostic.error().code() == CssErrorCode::InvalidPropertyValue
            && diagnostic.action() == CssRecoveryAction::DropDeclaration
    }));

    #[cfg(feature = "app-strict")]
    {
        let failure = surgeist_css::validate_style_attribute(source)
            .expect_err("strict validation reports every layered position failure");
        assert_eq!(failure.diagnostics(), report.diagnostics());
    }
}

#[test]
fn layered_position_i01_projection_is_exact_and_typed_calculations_are_current_only() {
    let report = parse_style_attribute(concat!(
        "background-position: left 10px top, bottom right; ",
        "background-position: left calc((1px + 2%) * 3) top; ",
        "mask-position: left top, 10% 20%; ",
        "mask-position: calc((1px + 2%) * 3) top",
    ));
    assert!(report.is_clean(), "{:?}", report.diagnostics());

    for (index, has_i01_projection) in [(0, true), (1, false), (2, true), (3, false)] {
        let value = report.syntax()[index]
            .known()
            .expect("known layered position")
            .property_value()
            .expect("ordinary layered position");
        let projection = match value {
            CssKnownPropertyValueRef::BackgroundPosition(value) => value.i01_subset(),
            CssKnownPropertyValueRef::MaskPosition(value) => value.i01_subset(),
            _ => panic!("expected layered position"),
        };
        assert_eq!(projection.is_some(), has_i01_projection, "index {index}");
    }
}
