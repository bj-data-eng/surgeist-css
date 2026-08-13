use surgeist_css::{CssFontFaceSource, CssRule, parse_sheet};

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
    assert!(matches!(sources[0], CssFontFaceSource::Local(_)));
    assert!(matches!(sources[1], CssFontFaceSource::Url(_)));
    assert!(matches!(sources[2], CssFontFaceSource::Url(_)));
    assert!(matches!(sources[3], CssFontFaceSource::Url(_)));
}
