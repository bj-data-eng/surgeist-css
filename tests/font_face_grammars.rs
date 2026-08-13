use surgeist_css::{
    CssErrorCode, CssFontFaceSource, CssFontFormatHint, CssFontTechHint, CssRecoveryAction,
    CssRule, parse_sheet,
};

fn assert_strict_parity(source: &str) {
    #[cfg(feature = "app-strict")]
    {
        let ordinary = parse_sheet(source);
        match surgeist_css::validate_sheet(source) {
            Ok(sheet) => {
                assert!(ordinary.is_clean());
                assert_eq!(&sheet, ordinary.syntax());
            }
            Err(failure) => assert_eq!(failure.diagnostics(), ordinary.diagnostics()),
        }
    }
    #[cfg(not(feature = "app-strict"))]
    let _ = source;
}

#[test]
fn font_sources_preserve_fonts3_formats_and_selected_fonts4_hints() {
    let source = concat!(
        "@font-face { font-family: Demo; src: ",
        "local(Installed Demo), ",
        "url(demo-a.bin) format(\"woff2\", \"opentype\"), ",
        "url(demo-b.bin) format(\"zebra\"), ",
        "url(demo-c.bin) format(woff2) tech(variations, color-colrv1); }",
    );
    let report = parse_sheet(source);

    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::FontFace(rule)] = report.syntax().rules() else {
        panic!("expected one retained font-face rule");
    };
    let sources = rule.descriptors().src().sources();
    assert_eq!(sources.len(), 4);
    let CssFontFaceSource::Local(local) = &sources[0] else {
        panic!("expected local source");
    };
    assert_eq!(local.as_str(), "Installed Demo");

    let CssFontFaceSource::Url(multiple) = &sources[1] else {
        panic!("expected first URL source");
    };
    assert_eq!(multiple.url(), "demo-a.bin");
    assert_eq!(
        multiple
            .formats()
            .unwrap()
            .formats()
            .iter()
            .map(|format| format.as_str())
            .collect::<Vec<_>>(),
        ["woff2", "opentype"]
    );
    assert_eq!(multiple.format(), None);

    let CssFontFaceSource::Url(arbitrary) = &sources[2] else {
        panic!("expected second URL source");
    };
    assert_eq!(arbitrary.formats().unwrap().formats()[0].as_str(), "zebra");
    assert_eq!(arbitrary.format(), None);

    let CssFontFaceSource::Url(keyword) = &sources[3] else {
        panic!("expected third URL source");
    };
    assert_eq!(keyword.formats().unwrap().formats()[0].as_str(), "woff2");
    assert_eq!(keyword.format(), Some(&CssFontFormatHint::Woff2));
    assert_eq!(
        keyword.tech(),
        &[CssFontTechHint::Variations, CssFontTechHint::ColorCOLRv1]
    );
    assert_strict_parity(source);
}

#[test]
fn font_face_family_and_local_names_distinguish_quoted_reserved_names() {
    let valid = concat!(
        "@font-face{font-family:\"serif\";src:",
        "local(\"inherit\"),local(Font Face),url(face.woff)}",
    );
    let report = parse_sheet(valid);
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::FontFace(rule)] = report.syntax().rules() else {
        panic!("expected font-face");
    };
    assert_eq!(rule.descriptors().font_family().as_str(), "serif");
    let [
        CssFontFaceSource::Local(global),
        CssFontFaceSource::Local(sequence),
        _,
    ] = rule.descriptors().src().sources()
    else {
        panic!("expected two local names and a URL");
    };
    assert_eq!(global.as_str(), "inherit");
    assert_eq!(sequence.as_str(), "Font Face");

    for source in [
        "@font-face{font-family:serif;src:url(face.woff)}.after{color:red}",
        "@font-face{font-family:Demo;src:local(inherit)}.after{color:red}",
        "@font-face{font-family:Demo;src:local(sans-serif)}.after{color:red}",
    ] {
        let report = parse_sheet(source);
        assert!(matches!(report.syntax().rules(), [CssRule::Style(_)]));
        assert_eq!(
            report
                .diagnostics()
                .iter()
                .map(|diagnostic| diagnostic.action())
                .collect::<Vec<_>>(),
            [
                CssRecoveryAction::DropDescriptor,
                CssRecoveryAction::DropAtRule
            ]
        );
        assert_strict_parity(source);
    }
}

#[test]
fn font_source_lists_reject_empty_items_and_invalid_hint_order() {
    let cases = [
        "@font-face{font-family:Demo;src:}",
        "@font-face{font-family:Demo;src:,url(a)}",
        "@font-face{font-family:Demo;src:url(a),}",
        "@font-face{font-family:Demo;src:mystery}",
        "@font-face{font-family:Demo;src:local()}",
        "@font-face{font-family:Demo;src:url(a) format()}",
        "@font-face{font-family:Demo;src:url(a) format(\"\")}",
        "@font-face{font-family:Demo;src:url(a) format(woff3)}",
        "@font-face{font-family:Demo;src:url(a) tech(variations) format(\"woff2\")}",
        "@font-face{font-family:Demo;src:url(a) format(\"woff2\") format(\"opentype\")}",
        "@font-face{font-family:Demo;src:url(a) tech(variations) tech(color-colrv1)}",
    ];
    for source in cases {
        let report = parse_sheet(source);
        assert!(report.syntax().rules().is_empty(), "{source}");
        assert_eq!(report.diagnostics().len(), 2, "{source}");
        assert_eq!(
            report.diagnostics()[0].error().code(),
            CssErrorCode::InvalidDescriptorValue,
            "{source}"
        );
        assert_eq!(
            report.diagnostics()[0].action(),
            CssRecoveryAction::DropDescriptor,
            "{source}"
        );
        assert_eq!(
            report.diagnostics()[1].action(),
            CssRecoveryAction::DropAtRule,
            "{source}"
        );
        assert_strict_parity(source);
    }
}

#[test]
fn selected_fonts4_format_and_technology_keywords_remain_ordered() {
    let source = concat!(
        "@font-face{font-family:Demo;src:",
        "url(a) format(woff),url(b) format(woff2),url(c) format(truetype),",
        "url(d) format(opentype),url(e) format(collection),",
        "url(f) format(embedded-opentype),url(g) format(svg),",
        "url(h) tech(variations,color-colrv0,color-colrv1,color-svg,color-sbix,",
        "color-cbdt,features-opentype,features-aat,features-graphite,incremental)}",
    );
    let report = parse_sheet(source);
    assert!(report.is_clean(), "{:?}", report.diagnostics());
    let [CssRule::FontFace(rule)] = report.syntax().rules() else {
        panic!("expected font-face");
    };
    let expected_formats = [
        CssFontFormatHint::Woff,
        CssFontFormatHint::Woff2,
        CssFontFormatHint::TrueType,
        CssFontFormatHint::OpenType,
        CssFontFormatHint::Collection,
        CssFontFormatHint::EmbeddedOpenType,
        CssFontFormatHint::Svg,
    ];
    for (source, expected) in rule.descriptors().src().sources()[..7]
        .iter()
        .zip(expected_formats)
    {
        let CssFontFaceSource::Url(source) = source else {
            panic!("expected URL source");
        };
        assert_eq!(source.format(), Some(&expected));
    }
    let CssFontFaceSource::Url(technology) = &rule.descriptors().src().sources()[7] else {
        panic!("expected technology URL source");
    };
    assert_eq!(
        technology.tech(),
        &[
            CssFontTechHint::Variations,
            CssFontTechHint::ColorCOLRv0,
            CssFontTechHint::ColorCOLRv1,
            CssFontTechHint::ColorSVG,
            CssFontTechHint::ColorSbix,
            CssFontTechHint::ColorCBDT,
            CssFontTechHint::FeaturesOpenType,
            CssFontTechHint::FeaturesAAT,
            CssFontTechHint::FeaturesGraphite,
            CssFontTechHint::Incremental,
        ]
    );
    assert_strict_parity(source);
}
