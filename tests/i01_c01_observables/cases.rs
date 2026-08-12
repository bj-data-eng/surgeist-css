#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntryPoint {
    Sheet,
    Style,
}

impl EntryPoint {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sheet => "sheet",
            Self::Style => "style",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureMode {
    Both,
    AppStrict,
}

impl FeatureMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Both => "both",
            Self::AppStrict => "app-strict",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Case {
    pub id: String,
    pub owner: String,
    pub entry: EntryPoint,
    pub feature: FeatureMode,
    pub input: String,
}

impl Case {
    pub fn new(
        id: impl Into<String>,
        owner: impl Into<String>,
        entry: EntryPoint,
        feature: FeatureMode,
        input: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            owner: owner.into(),
            entry,
            feature,
            input: input.into(),
        }
    }
}

fn case(
    id: impl Into<String>,
    owner: impl Into<String>,
    entry: EntryPoint,
    input: impl Into<String>,
) -> Case {
    Case::new(id, owner, entry, FeatureMode::Both, input)
}

pub fn non_property_cases() -> Vec<Case> {
    use EntryPoint::{Sheet, Style};
    let mut cases = Vec::new();
    macro_rules! pair {
        ($id:literal, $entry:ident, $positive:literal, $boundary:literal) => {{
            cases.push(case(
                concat!("catalog.non-property.", $id, ".positive"),
                concat!("conformance_catalog::", $id, "/positive"),
                $entry,
                $positive,
            ));
            cases.push(case(
                concat!("catalog.non-property.", $id, ".boundary"),
                concat!("conformance_catalog::", $id, "/boundary"),
                $entry,
                $boundary,
            ));
        }};
    }
    macro_rules! positive {
        ($id:literal, $entry:ident, $input:literal) => {
            cases.push(case(
                concat!("catalog.non-property.", $id, ".positive"),
                concat!("conformance_catalog::", $id, "/positive"),
                $entry,
                $input,
            ));
        };
    }
    macro_rules! boundary {
        ($id:literal, $entry:ident, $input:literal) => {
            cases.push(case(
                concat!("catalog.non-property.", $id, ".boundary"),
                concat!("conformance_catalog::", $id, "/boundary"),
                $entry,
                $input,
            ));
        };
    }
    pair!(
        "baseline.rule.import",
        Sheet,
        "@import \"theme.css\";",
        "@import url(theme.css) supports(display: grid);"
    );
    pair!(
        "baseline.rule.layer-statement",
        Sheet,
        "@layer reset, theme;",
        "@layer initial;"
    );
    pair!(
        "baseline.rule.layer-block",
        Sheet,
        "@layer theme { .x { color: red; } }",
        "@layer first, second { .x { color: red; } }"
    );
    pair!(
        "baseline.rule.font-face",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); }",
        "@font-face named { font-family: Inter; src: url(inter.woff2); }"
    );
    pair!(
        "baseline.rule.keyframes",
        Sheet,
        "@keyframes fade { from { opacity: 0; } to { opacity: 1; } }",
        "@keyframes none { from { opacity: 0; } }"
    );
    pair!(
        "baseline.rule.style",
        Sheet,
        ".x { color: red; }",
        "??? { color: red; }"
    );
    pair!(
        "baseline.rule.media",
        Sheet,
        "@media screen { .x { color: red; } }",
        "@media (width: calc(1px)) { .x { color: red; } }"
    );
    pair!(
        "baseline.rule.container",
        Sheet,
        "@container (width > 1px) { .x { color: red; } }",
        "@container scroll-state(stuck: top) { .x { color: red; } }"
    );
    pair!(
        "baseline.rule.scope",
        Sheet,
        "@scope (.card) { .title { color: red; } }",
        "@scope .card { .title { color: red; } }"
    );
    positive!(
        "foundation.encoding.charset",
        Sheet,
        "@charset \"UTF-8\"; .x { color: red; }"
    );
    positive!(
        "foundation.declaration-list.style-attribute",
        Style,
        "color: red; width: 1px"
    );
    positive!(
        "foundation.declaration.importance",
        Style,
        "color: red !important"
    );
    pair!(
        "baseline.declaration.custom-property",
        Style,
        "--theme: dark",
        "--x: inherit 1px"
    );
    pair!(
        "baseline.value.substitution-dependent",
        Style,
        "width: var(--width, 1px)",
        "width: var(color)"
    );
    boundary!(
        "later.rule.namespace",
        Sheet,
        "@namespace svg url(https://example.test/svg);"
    );
    boundary!(
        "later.rule.supports",
        Sheet,
        "@supports (display: grid) { .x { color: red; } }"
    );
    boundary!(
        "later.rule.counter-style",
        Sheet,
        "@counter-style thumbs { system: cyclic; symbols: 👍; suffix: \" \"; }"
    );
    boundary!("later.rule.page", Sheet, "@page { margin: 1cm; }");
    boundary!(
        "later.rule.font-feature-values",
        Sheet,
        "@font-feature-values Font One { @styleset { nice: 1; } }"
    );
    pair!(
        "baseline.descriptor.font-family",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); }",
        "@font-face { font-family: serif, sans-serif; src: url(inter.woff2); }"
    );
    pair!(
        "baseline.descriptor.src",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2) format(woff2); }",
        "@font-face { font-family: Inter; src: url(inter.woff2) format(woff3); }"
    );
    pair!(
        "baseline.descriptor.font-weight",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-weight: 400 700; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-weight: bolder; }"
    );
    pair!(
        "baseline.descriptor.font-style",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-style: italic; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-style: made-up; }"
    );
    pair!(
        "baseline.descriptor.font-stretch",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-stretch: 75% 125%; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-stretch: wide; }"
    );
    pair!(
        "baseline.descriptor.font-display",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); font-display: swap; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); font-display: made-up; }"
    );
    pair!(
        "baseline.descriptor.unicode-range",
        Sheet,
        "@font-face { font-family: Inter; src: url(inter.woff2); unicode-range: U+0000-00FF; }",
        "@font-face { font-family: Inter; src: url(inter.woff2); unicode-range: U+110000-110001; }"
    );
    pair!(
        "baseline.selector.complex",
        Sheet,
        "article#main.card[data-ready][lang|=\"en\"] > span + a ~ b { color: red; }",
        "svg|a { color: red; }"
    );
    pair!(
        "baseline.selector.pseudo-class",
        Sheet,
        ".button:hover { color: red; }",
        ".link:visited { color: red; }"
    );
    pair!(
        "baseline.selector.functional",
        Sheet,
        ".item:nth-child(2n+1) { color: red; }",
        ".item:lang(en) { color: red; }"
    );
    pair!(
        "baseline.selector.extension-state",
        Sheet,
        ".button:focus-visible { color: red; }",
        ".target:target { color: red; }"
    );
    pair!(
        "baseline.selector.extension-functional",
        Sheet,
        ".item:is(.primary, .secondary) { color: red; }",
        ".item:has(:has(.nested)) { color: red; }"
    );
    pair!(
        "baseline.selector.attribute-case",
        Sheet,
        "[data-kind=\"primary\" i] { color: red; }",
        "[data-kind=\"primary\" q] { color: red; }"
    );
    pair!(
        "baseline.selector.pseudo-element",
        Sheet,
        ".item::before { content: \"x\"; }",
        ".item::first-line { color: red; }"
    );
    pair!(
        "baseline.selector.nesting",
        Sheet,
        ".card { & > .title { color: red; } }",
        ".card { & || .title { color: red; } }"
    );
    pair!(
        "baseline.media.query-list",
        Sheet,
        "@media screen and (min-width: 1px), print { .x { color: red; } }",
        "@media screen, ??? { .x { color: red; } }"
    );
    pair!(
        "baseline.media.type",
        Sheet,
        "@media print { .x { color: red; } }",
        "@media speech { .x { color: red; } }"
    );
    pair!(
        "baseline.media.range-feature",
        Sheet,
        "@media (width >= 1px) { .x { color: red; } }",
        "@media (device-width: 1px) { .x { color: red; } }"
    );
    pair!(
        "baseline.media.discrete-feature",
        Sheet,
        "@media (orientation: landscape) { .x { color: red; } }",
        "@media (scripting: enabled) { .x { color: red; } }"
    );
    pair!(
        "baseline.container.condition",
        Sheet,
        "@container (width > 1px) and style(--theme) { .x { color: red; } }",
        "@container style(color: red) { .x { color: red; } }"
    );
    pair!(
        "baseline.container.size-feature",
        Sheet,
        "@container (inline-size > 1px) { .x { color: red; } }",
        "@container (unknown-size > 1px) { .x { color: red; } }"
    );
    cases
}

pub fn focused_cases() -> Vec<Case> {
    use EntryPoint::{Sheet, Style};
    let mut cases = Vec::new();
    macro_rules! c {
        ($id:literal, $owner:literal, $entry:ident, $input:expr) => {
            cases.push(case(concat!("focused.", $id), $owner, $entry, $input));
        };
    }
    // Public front doors and the app-strict-specific consumer cases.
    c!(
        "public.sheet.clean",
        "public_surface::public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery/clean",
        Sheet,
        ".clean { color: red; }"
    );
    c!(
        "public.sheet.recovered",
        "public_surface::public_surface_sheet_reports_expose_retained_syntax_and_structured_recovery/recovered",
        Sheet,
        ".before { color: red; } @unknown x; .after { color: blue; }"
    );
    c!(
        "public.style.authored",
        "public_surface::public_surface_style_attributes_preserve_importance_custom_and_substitution_syntax",
        Style,
        "color: red; --Theme: RGB(1, 2, var(--fallback)); width: var(--size, 2px) !important; mystery: 1"
    );
    c!(
        "public.non-bmp",
        "public_surface::public_surface_non_bmp_coordinates_are_byte_line_and_utf16_based",
        Sheet,
        ".😀 { mystery: 1; color: red; }"
    );
    for (suffix, input) in [
        ("drop-declaration", ".x { mystery: 1; color: red; }"),
        (
            "drop-descriptor",
            "@font-face { font-family: Demo; src: url(demo); mystery: 1; }",
        ),
        (
            "drop-qualified-rule",
            "??? { color: red; } .after { color: blue; }",
        ),
        ("drop-at-rule", "@unknown value;"),
        (
            "drop-keyframe-block",
            "@keyframes fade { fn(a) { opacity: .5; } to { opacity: 1; } }",
        ),
        ("drop-selector-list-item", ":is(.kept,???) { color: red; }"),
        ("replace-media", "@media screen, ??? { .x { color: red; } }"),
        ("implicit-closure", ".x { color: red;"),
        ("ignore-legacy", "<!-- .x { color: red; }"),
    ] {
        cases.push(case(
            format!("focused.public.actions.{suffix}"),
            format!("public_surface::public_surface_emits_all_ten_recovery_actions/{suffix}"),
            Sheet,
            input,
        ));
    }
    let over_limit = format!(
        "{}{}{} {{ color: red; }}",
        ":is(".repeat(257),
        ".leaf",
        ")".repeat(257)
    );
    cases.push(case(
        "focused.public.actions.stop-at-limit",
        "public_surface::public_surface_emits_all_ten_recovery_actions/stop-at-limit",
        Sheet,
        over_limit,
    ));
    c!(
        "public.strict.style",
        "public_surface::public_surface_enabled_validators_accept_clean_reports_and_preserve_failures",
        Style,
        "color: red; mystery: 1"
    );
    // Focused style-attribute recovery table and boundary families.
    for (index, input) in ["", " \t/**/\n", "color: red", "color: red;"]
        .into_iter()
        .enumerate()
    {
        cases.push(case(format!("focused.style-attribute.clean.{index:02}"), format!("style_attribute_recovery::style_attribute_empty_trivia_and_optional_final_semicolon_are_clean/{index}"), Style, input));
    }
    c!(
        "style-attribute.parity",
        "style_attribute_recovery::style_attribute_ordinary_custom_global_substitution_and_importance_match_style_blocks",
        Style,
        "width: 2px; --Theme: ready; color: inherit; height: var(--h, 8px) !important"
    );
    for (index, unit) in [
        "mystery: 1;",
        "width: nope;",
        "width: 2px !oops;",
        "color: #ggg;",
        "--bad name: 1px;",
        "--x: inherit 1px;",
        "broken;",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.style-attribute.error-class.{index:02}"), format!("style_attribute_recovery::style_attribute_declaration_error_classes_match_style_blocks_modulo_wrapper_offset/{index}"), Style, format!("color: red; {unit} height: 3px;")));
    }
    c!(
        "style-attribute.non-declarations",
        "style_attribute_recovery::style_attribute_non_declaration_units_drop_independently_in_source_order",
        Style,
        "@unknown x; color: red; .nested { width: 1px; } opacity: 1; broken; height: 2px; ,; --kept: yes;"
    );
    c!(
        "style-attribute.closers",
        "style_attribute_recovery::style_attribute_block_at_rules_and_malformed_closers_drop_without_hiding_later_values",
        Style,
        "@unknown screen { color: red; } } color: red; ) width: 2px; ] height: 3px;"
    );
    c!(
        "style-attribute.implicit",
        "style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/implicit",
        Style,
        "--value: fn([x"
    );
    c!(
        "style-attribute.bad-url",
        "style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/bad-url",
        Style,
        "background-image: url(bad url"
    );
    for depth in [255_usize, 256, 257] {
        cases.push(case(format!("focused.style-attribute.depth.{depth}"), format!("style_attribute_recovery::style_attribute_component_eof_closure_and_nesting_limit_match_shared_boundaries/{depth}"), Style, format!("--deep: {}x{}", "f(".repeat(depth), ")".repeat(depth))));
    }
    // Specialized list-recovery concrete tables.
    for (index, input) in [
        ":is(???,.a,.b) { color: red; }",
        ":is(.a,???,.b) { color: red; }",
        ":is(.a,.b,???) { color: red; }",
        ":where(???) { color: red; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.specialized.forgiving.{index:02}"), format!("specialized_list_recovery::specialized_list_forgiving_selector_members_drop_independently_in_authored_order/{index}"), Sheet, input));
    }
    c!(
        "specialized.forgiving-empty",
        "specialized_list_recovery::specialized_list_empty_forgiving_member_uses_its_delimiting_comma_span",
        Sheet,
        ":is(.a,,.b) { color: red; }"
    );
    c!(
        "specialized.forgiving-balanced",
        "specialized_list_recovery::specialized_list_forgiving_recovery_stops_at_balanced_nested_commas",
        Sheet,
        ":is(???(a,b),.ok) { color: red; }"
    );
    for (index, input) in [
        ":not(.a,???,.b) { color: red; }",
        ":has(.a,???,.b) { color: red; }",
        ":nth-child(2n of .a,???,.b) { color: red; }",
        ".a,???,.b { color: red; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.specialized.unforgiving.{index:02}"), format!("specialized_list_recovery::specialized_list_not_has_nth_and_ordinary_selector_lists_remain_unforgiving/{index}"), Sheet, input));
    }
    for (index, input) in [
        "@media ???,screen,print { .x { color: red; } }",
        "@media screen,???,print { .x { color: red; } }",
        "@media screen,print,??? { .x { color: red; } }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.specialized.media.{index:02}"), format!("specialized_list_recovery::specialized_list_media_members_become_never_in_authored_order/{index}"), Sheet, input));
    }
    for (id, input) in [
        ("repeated-selector", ":is(???,.ok,???) { color: red; }"),
        (
            "repeated-media",
            "@media ???,screen,,print,??? { .x { color: red; } }",
        ),
        ("empty-media", "@media screen,,print { .x { color: red; } }"),
        (
            "balanced-media",
            "@media ???(a,b),screen { .x { color: red; } }",
        ),
        (
            "media-position",
            "@media screen,(unknown: yes),print { .x { color: red; } }",
        ),
        ("import", "@import \"theme.css\" ???,screen;"),
        ("nested", ".parent { &:is(.ok,???) { color: red; } }"),
        (
            "scoped",
            "@scope (:is(.root,???)) { :where(.kept,???) { color: red; } @media ???,screen {} }",
        ),
        (
            "non-bmp",
            "@media screen, /*😀*/ ???, print { .x { color: red; } }",
        ),
        (
            "clean-media",
            "@media screen,(width: 1px) { .x { color: red; } }",
        ),
        (
            "delegation",
            "@media /*😀*/ not (width: 1px), only screen, ??? { .x { color: red; } }",
        ),
    ] {
        cases.push(case(
            format!("focused.specialized.{id}"),
            format!("specialized_list_recovery::{id}"),
            Sheet,
            input,
        ));
    }
    // App-strict public cases have their own feature identity while all ordinary cases also run under the feature.
    for (id, entry, input) in [
        ("clean-sheet", Sheet, ".x { color: red; }"),
        ("recovered-sheet", Sheet, ".x { mystery: 1; }"),
        ("clean-style", Style, "color: red"),
        ("recovered-style", Style, "mystery: 1"),
        (
            "multi-sheet",
            Sheet,
            "<!-- .x { mystery: 1; width: nope; } -->",
        ),
        ("multi-style", Style, "mystery: 1; width: nope; color: red"),
        ("never", Sheet, "@media screen, ??? { .x { color: red; } }"),
        ("implicit-sheet", Sheet, ".x { color: red;"),
        ("implicit-style", Style, "--x: f(value"),
    ] {
        cases.push(Case::new(
            format!("focused.app-strict.{id}"),
            format!("app_strict_parity::{id}"),
            entry,
            FeatureMode::AppStrict,
            input,
        ));
    }
    // Structured diagnostic public cases.
    for (index, input) in [
        "@not-a-css-rule;",
        "@supports (display: grid) {}",
        ".x { width: 1px; } @import 'x.css';",
        "@font-face nope {}",
        "@media screen;",
        "@font-face {\n}",
        "@layer 😀, theme {}",
        "??? { width: 1px; }",
        "@media (unknown: yes) { .x { width: 1px; } }",
        "x;",
        ".x { WIDHT: 1px; }",
        "@font-face { mystery: x; font-family: Test; src: url(test.woff2); }",
        "@font-face { font-family: One; font-family: Two; src: url(test.woff2); }",
        ".x { color: #ggg; }",
        ".x { width: 1px !oops; }",
        ".panel { WIDTH: n\\6f pe; }",
        ".panel { width:",
        ".panel { width 1px; }",
        "??? { width: 1px; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.structured-errors.{index:02}"),
            format!("structured_errors::case/{index}"),
            Sheet,
            input,
        ));
    }
    // Source-coordinate and coupled declaration cases.
    for (index, input) in [
        ".a { width: 1px; }",
        ".a {\n  width: 1px; }",
        ".a {\r\n  width: 1px; }",
        ".\\61 bc { width: 1px; }",
        ".😀2 { width: 1px; }",
        "/*a\nbc*/@import \"theme.css\";",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(format!("focused.source-coordinates.{index:02}"), format!("source_coordinates::source_public_nodes_expose_zero_based_byte_line_and_utf16_coordinates/{index}"), Sheet, input));
    }
    for (index, input) in [
        ".x { width: 1px; color: red; opacity: .5; }",
        ".x { width: 1px; width: inherit; width: var(--x); }",
        ".x { all: inherit; all: var(--x); }",
        ".x { --Theme: RGB(1, 2, var(--fallback)); }",
        ".x { WIDTH: 1px; }",
        ".x { color: red; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.coupled.{index:02}"),
            format!("coupled_declarations::case/{index}"),
            Sheet,
            input,
        ));
    }
    // Declaration importance and authored-value concrete boundaries.
    for (index, input) in [
        ".x { color: red; width: 1px !important; }",
        ".x { --x: bang!important; --y: f(!important) !important; }",
        ".x { width: 1px !oops; }",
        ".x { color: red; width: 1px !oops; height: 2px; }",
        "@keyframes fade { from { opacity: 0 } to { opacity: 1; } }",
        "@keyframes fade { from { opacity: 0 !important; } }",
        "@keyframes fade { from { --x: value !important; } }",
        "@font-face { font-family: Demo; src: url(demo.woff2); font-display: swap; }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.importance.{index:02}"),
            format!("declaration_importance::case/{index}"),
            Sheet,
            input,
        ));
    }
    for (index, input) in [
        ".x { --empty:; --space:   ; }",
        ".x { --Theme: RGB(1, 2, var(--fallback)); }",
        ".x { --a: inherit; --b: initial; --c: unset; --d: revert; --e: revert-layer; }",
        ".x { width: 1px; height: var(--h, 8px); color: red; }",
        ".x { width: var(--x, red); }",
        ".x { width: var(--x, red; blue); }",
        ".x { width: var(--x, red ! blue); }",
        ".x { --x: fn([a;b!important]); }",
        ".x { --bad name: 1px; }",
        ".x { width: var(color); }",
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.authored-values.{index:02}"),
            format!("authored_declaration_values::case/{index}"),
            Sheet,
            input,
        ));
    }
    // Core stylesheet recovery concrete rows and finite encoding tables.
    for (index, input) in [
        "", "<!-- .before { color: red; } <!-- .after { color: blue; }", "--> .before { color: red; } --> .after { color: blue; }",
        "; .after { color: blue; }", ".before { color: red; } ; .after { color: blue; }", "} .after { color: blue; }",
        ".before { color: red; } } .after { color: blue; }", "; @charset \"UTF-8\"; .after { color: blue; }",
        "} @charset \"UTF-8\"; .after { color: blue; }", " \n/**/ <!-- --> \t",
        ".before { color: red; } @mystery one(foo; bar) { nested: {x; y}; } .after { color: blue; }",
        ".before { color: red; } @namespace svg url(http://example.test/a;b); .after { color: blue; }",
        ".before { color: red; } ??? { width: 1px; nested: fn({x;y}); } .after { color: blue; }",
        "\u{feff} /* leading */ @charset \"UTF-8\"; .after { color: blue; }",
        "@charset UTF-8; .after { color: blue; }", "@charset \"\"; .after { color: blue; }",
        "@charset \"UTF-8\" { ignored; } .after { color: blue; }", "@charset UTF-8;", "@charset \"\";",
        "@charset 'UTF-8';", "@charset /*comment*/ 'UTF-8';", "@charset \"UTF-8\"", "@charset \"UTF-8\" {}",
    ].into_iter().enumerate() { cases.push(case(format!("focused.stylesheet-recovery.{index:02}"), format!("stylesheet_recovery::case/{index}"), Sheet, input)); }
    // Initiative audit public-report stimuli (duplicates remain distinct owner identities).
    for (index, (entry, input)) in [
        (Sheet, ".before { color: red; } @unknown x; .middle { mystery: 1; width: 2px; } ??? { color: black; } .after { height: 3px; }"),
        (Style, "color: red; broken; width: 2px"), (Sheet, ".😀 { mystery: 1; width: bogus; color: red; } @unknown x;"),
        (Style, "--Theme: RGB(1, 2, var(--fallback)); width: var(--size, 2px) !important; color: red"),
        (Sheet, "@charset \"UTF-8\"; .x { color: red; }"), (Sheet, ".kept { color: red; } @unknown x;"),
        (Style, "--Theme: var(--fallback); width: 2px !important"), (Sheet, "@media screen, ??? { .x { color: red; } }"),
    ].into_iter().enumerate() { cases.push(case(format!("focused.initiative-audit.{index:02}"), format!("initiative_i01_audit::case/{index}"), entry, input)); }
    // Structural-limit loop families use explicit deterministic inputs at every tested boundary.
    for depth in [255_usize, 256, 257] {
        cases.push(case(format!("focused.structural.layers.{depth}"), format!("structural_recovery_adversarial::structural_recovery_accepts_256_rule_blocks_and_drops_only_level_257/{depth}"), Sheet, format!("{}{}.after{{color:red}}", "@layer{".repeat(depth), "}".repeat(depth))));
        for (kind, opener, closer) in [
            ("function", "f(", ")"),
            ("paren", "(", ")"),
            ("square", "[", "]"),
            ("curly", "{", "}"),
        ] {
            let component_depth = depth.saturating_sub(1);
            cases.push(case(format!("focused.structural.component.{kind}.{depth}"), format!("structural_recovery_adversarial::structural_recovery_shares_rule_and_component_depth_for_functions_and_blocks/{kind}/{depth}"), Sheet, format!(".target{{--x:{}x{};color:blue}}.after{{color:red}}", opener.repeat(component_depth), closer.repeat(component_depth))));
        }
    }
    cases.push(case("focused.structural.layers.eof-257", "structural_recovery_adversarial::structural_recovery_nesting_limit_at_eof_spans_remaining_bounded_unit", Sheet, "@layer{".repeat(257)));
    for (index, input) in [
        "/*({[({[*/.target{--x:\"})]})]\\\"tail\";--y:ident\\(\\[\\{;color:blue}.after{color:red}"
            .to_owned(),
        format!(
            "{} .x{{color:red}} {}",
            "@scope{".repeat(255),
            "}".repeat(255)
        ),
        format!(
            "{}color:red{}}}.after{{color:blue}}",
            ".x{".repeat(256),
            "}".repeat(255)
        ),
        "@keyframes fade { from { mystery: 1; } } .after { color: red; }".to_owned(),
        "".to_owned(),
        ";;;;;}}}}\0\u{fffd}".to_owned(),
        "🦊💥\n@unknown fn({a;b}); .after{color:red}".to_owned(),
        format!("{}{}", "@bad{};".repeat(256), ".after{color:red}"),
    ]
    .into_iter()
    .enumerate()
    {
        cases.push(case(
            format!("focused.structural.misc.{index:02}"),
            format!("structural_recovery_adversarial::misc/{index}"),
            Sheet,
            input,
        ));
    }
    // App-strict generated depth rows.
    let mut structural = ".x{".repeat(257);
    structural.push_str("color:red;");
    structural.push_str(&"}".repeat(257));
    cases.push(Case::new(
        "focused.app-strict.structural-depth",
        "app_strict_parity::structural-depth",
        Sheet,
        FeatureMode::AppStrict,
        structural,
    ));
    let selector = format!(
        "{}{}{}{{color:red}}",
        ":is(".repeat(257),
        ".leaf",
        ")".repeat(257)
    );
    cases.push(Case::new(
        "focused.app-strict.selector-depth",
        "app_strict_parity::selector-depth",
        Sheet,
        FeatureMode::AppStrict,
        selector,
    ));
    let style = format!("--x:{}x{}", "f(".repeat(257), ")".repeat(257));
    cases.push(Case::new(
        "focused.app-strict.style-depth",
        "app_strict_parity::style-depth",
        Style,
        FeatureMode::AppStrict,
        style,
    ));
    // Nested structural recovery cases, including each generated group-context input.
    let failed_at = "@mystery fn({x; y}) { .lost { color: black; } }";
    for (kind, source) in [
        (
            "layer",
            format!(
                "@layer theme {{ .before {{ color: red; }} {failed_at} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "media",
            format!(
                "@media screen {{ .before {{ color: red; }} {failed_at} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "container",
            format!(
                "@container (width > 1px) {{ .before {{ color: red; }} {failed_at} .after {{ color: blue; }} }}"
            ),
        ),
    ] {
        cases.push(case(format!("focused.nested-structural.group.{kind}"), format!("nested_structural_recovery::nested_structural_group_contexts_retain_siblings_around_balanced_at_rule_failure/{kind}"), Sheet, source));
    }
    let failed_rule = ".bad:is(.one, .two), { color: black; }";
    for (kind, source) in [
        (
            "group",
            format!(
                "@media screen {{ .before {{ color: red; }} {failed_rule} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "scope",
            format!(
                "@scope {{ .before {{ color: red; }} {failed_rule} .after {{ color: blue; }} }}"
            ),
        ),
        (
            "style",
            format!(
                ".host {{ color: red; & .before {{ width: 1px; }} {failed_rule} & .after {{ height: 2px; }} opacity: 1; }}"
            ),
        ),
    ] {
        cases.push(case(format!("focused.nested-structural.qualified.{kind}"), format!("nested_structural_recovery::nested_structural_qualified_failures_recover_in_group_scope_and_style_contexts/{kind}"), Sheet, source));
    }
    for (id, input) in [
        (
            "repeated",
            "@layer empty { @one fn({a; b}); .bad, { color: red; } } .after { color: blue; }",
        ),
        (
            "style-at-rule",
            ".host { color: red; & .before { width: 1px; } @mystery fn({x; y}); & .after { height: 2px; } opacity: 1; }",
        ),
        (
            "scope-at-rule",
            "@scope { .before { color: red; } @mystery fn({x; y}); .after { color: blue; } }",
        ),
        (
            "keyframes-recover",
            "@keyframes fade { from { opacity: 0; mystery: fn({a; b}); width: 1px; } 55 { opacity: .5; } to { opacity: 1; } } .after { color: blue; }",
        ),
        (
            "keyframes-child-loss",
            "@keyframes fade { from { mystery: 1; } } .after { color: blue; }",
        ),
        (
            "keyframes-balanced",
            "@keyframes fade { from { opacity: 0; } 25% { opacity: .25; @media fn(a, b) { width: 1px; } height: 2px; } fn(a, b) { opacity: .5; } to { opacity: 1; } }",
        ),
    ] {
        cases.push(case(
            format!("focused.nested-structural.{id}"),
            format!("nested_structural_recovery::{id}"),
            Sheet,
            input,
        ));
    }
    cases
}
