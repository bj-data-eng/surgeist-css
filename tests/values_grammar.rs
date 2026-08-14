use surgeist_css::{
    CssImageValue, CssKnownPropertyValueRef, CssSupportStatus, feature_metadata,
    parse_style_attribute,
};

#[test]
fn c14_remaining_shared_values_are_typed() {
    let report = parse_style_attribute(
        "background-image: url(\"theme.css\" integrity(sha256) cors); width: 2px",
    );
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    assert_eq!(report.syntax().len(), 2);

    let CssKnownPropertyValueRef::BackgroundImage(background) = report.syntax()[0]
        .known()
        .expect("known background-image")
        .property_value()
        .expect("ordinary background-image")
    else {
        panic!("expected background-image");
    };
    assert!(matches!(
        background.images().images(),
        [CssImageValue::Url(url)] if url.as_str() == "theme.css"
    ));

    for id in [
        "official.value.syntax-token-stream",
        "official.value.component-value",
        "official.value.simple-block",
        "official.value.function",
        "official.value.declaration-value",
        "official.value.any-value",
        "official.value.an-plus-b",
        "official.value.unicode-range",
        "official.value.css-wide-keyword",
        "official.value.custom-ident",
        "official.value.ident",
        "official.value.string",
        "official.value.url",
        "official.value.url-modifier",
    ] {
        let metadata = feature_metadata(id).unwrap_or_else(|| panic!("missing metadata for {id}"));
        assert_eq!(metadata.status(), CssSupportStatus::Complete, "{id}");
        assert_eq!(metadata.supported_subset(), None, "{id}");
        assert_eq!(metadata.unsupported_remainder(), None, "{id}");
    }
}
