use std::path::Path;

const ORACLE_PATH: &str = "tests/i01_c01_observables.rs";
const MIGRATED_HELPER_PATHS: &[&str] = &[
    "src/test_support.rs",
    "src/tests.rs",
    "tests/authored_declaration_values.rs",
    "tests/coupled_declarations.rs",
    "tests/declaration_importance.rs",
    ORACLE_PATH,
    "tests/initiative_i01_audit.rs",
    "tests/property_schema.rs",
    "tests/public_surface.rs",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Span {
    start: usize,
    end: usize,
}

fn source(path: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn rust_code_without_comments_or_literals(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut code = bytes.to_vec();
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index..] {
            [b'/', b'/', ..] => {
                let start = index;
                index += 2;
                while index < bytes.len() && bytes[index] != b'\n' {
                    index += 1;
                }
                code[start..index].fill(b' ');
            }
            [b'/', b'*', ..] => {
                let start = index;
                index += 2;
                let mut depth = 1usize;
                while index < bytes.len() && depth != 0 {
                    if bytes[index..].starts_with(b"/*") {
                        depth += 1;
                        index += 2;
                    } else if bytes[index..].starts_with(b"*/") {
                        depth -= 1;
                        index += 2;
                    } else {
                        index += 1;
                    }
                }
                assert_eq!(depth, 0, "unterminated block comment");
                code[start..index].fill(b' ');
            }
            [b'b', b'\'', ..] | [b'\'', ..]
                if bytes[index] == b'b'
                    || bytes.get(index + 2) == Some(&b'\'')
                    || bytes.get(index + 1) == Some(&b'\\') =>
            {
                let start = index;
                index += usize::from(bytes[start] == b'b') + 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        index += 2;
                    } else {
                        let byte = bytes[index];
                        index += 1;
                        if byte == b'\'' {
                            break;
                        }
                    }
                }
                code[start..index.min(bytes.len())].fill(b' ');
            }
            [b'b', b'"', ..] | [b'"', ..] => {
                let start = index;
                index += usize::from(bytes[start] == b'b') + 1;
                while index < bytes.len() {
                    if bytes[index] == b'\\' {
                        index += 2;
                    } else {
                        let byte = bytes[index];
                        index += 1;
                        if byte == b'"' {
                            break;
                        }
                    }
                }
                code[start..index.min(bytes.len())].fill(b' ');
            }
            [b'b', b'r', ..] | [b'r', ..] => {
                let start = index;
                let r = index + usize::from(bytes[index] == b'b');
                let mut quote = r + 1;
                while quote < bytes.len() && bytes[quote] == b'#' {
                    quote += 1;
                }
                if quote >= bytes.len() || bytes[quote] != b'"' {
                    index += 1;
                    continue;
                }
                let hashes = quote - (r + 1);
                index = quote + 1;
                loop {
                    let Some(relative) = bytes[index..].iter().position(|byte| *byte == b'"')
                    else {
                        panic!("unterminated raw string literal");
                    };
                    let end_quote = index + relative;
                    if bytes.get(end_quote + 1..end_quote + 1 + hashes)
                        == Some(&bytes[r + 1..quote])
                    {
                        index = end_quote + 1 + hashes;
                        break;
                    }
                    index = end_quote + 1;
                }
                code[start..index].fill(b' ');
            }
            _ => index += 1,
        }
    }
    String::from_utf8(code).expect("masking preserves UTF-8")
}

fn balanced_span(code: &str, open: usize) -> Span {
    let bytes = code.as_bytes();
    let opening = bytes[open];
    let closing = match opening {
        b'(' => b')',
        b'{' => b'}',
        b'[' => b']',
        _ => panic!("expected an opening delimiter"),
    };
    let mut depth = 0usize;
    for (offset, byte) in bytes[open..].iter().enumerate() {
        if *byte == opening {
            depth += 1;
        } else if *byte == closing {
            depth -= 1;
            if depth == 0 {
                return Span {
                    start: open,
                    end: open + offset + 1,
                };
            }
        }
    }
    panic!("unbalanced delimiter at byte {open}")
}

fn item_span(code: &str, marker: &str) -> Span {
    let start = code
        .find(marker)
        .unwrap_or_else(|| panic!("missing item marker `{marker}`"));
    let open = code[start..]
        .find('{')
        .map(|offset| start + offset)
        .expect("item has a body");
    let body = balanced_span(code, open);
    Span {
        start,
        end: body.end,
    }
}

fn function_spans(code: &str) -> Vec<Span> {
    let bytes = code.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0usize;
    while let Some(relative) = code[index..].find("fn ") {
        let start = index + relative;
        if start != 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
            index = start + 3;
            continue;
        }
        let Some(relative_open) = code[start..].find('{') else {
            break;
        };
        let body = balanced_span(code, start + relative_open);
        spans.push(Span {
            start,
            end: body.end,
        });
        index = body.end;
    }
    spans
}

fn braced_type_spans(code: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    for marker in ["enum ", "struct ", "trait "] {
        let mut index = 0usize;
        while let Some(relative) = code[index..].find(marker) {
            let start = index + relative;
            let prefix_is_identifier = start != 0
                && (code.as_bytes()[start - 1].is_ascii_alphanumeric()
                    || code.as_bytes()[start - 1] == b'_');
            if prefix_is_identifier {
                index = start + marker.len();
                continue;
            }
            let tail = &code[start..];
            let open = tail.find('{');
            let semicolon = tail.find(';');
            if let Some(open) = open.filter(|open| semicolon.is_none_or(|end| *open < end)) {
                let body = balanced_span(code, start + open);
                spans.push(Span {
                    start,
                    end: body.end,
                });
                index = body.end;
            } else {
                index = start + marker.len();
            }
        }
    }
    spans
}

fn string_literal_spans(source: &str) -> Vec<Span> {
    let code = rust_code_without_comments_or_literals(source);
    let bytes = source.as_bytes();
    let masked = code.as_bytes();
    let mut spans = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'"' || masked[index] != b' ' {
            index += 1;
            continue;
        }
        let start = index;
        let mut prefix = start;
        while prefix > 0 && bytes[prefix - 1] == b'#' {
            prefix -= 1;
        }
        let raw = prefix > 0 && bytes[prefix - 1] == b'r';
        let hashes = start - prefix;
        index += 1;
        if raw {
            loop {
                let relative = bytes[index..]
                    .iter()
                    .position(|byte| *byte == b'"')
                    .expect("terminated raw string literal");
                let quote = index + relative;
                if bytes.get(quote + 1..quote + 1 + hashes) == Some(&bytes[prefix..start]) {
                    index = quote + 1 + hashes;
                    break;
                }
                index = quote + 1;
            }
        } else {
            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index += 2;
                } else {
                    let byte = bytes[index];
                    index += 1;
                    if byte == b'"' {
                        break;
                    }
                }
            }
        }
        spans.push(Span { start, end: index });
    }
    spans
}

fn has_debug_format_spec(literal: &str) -> bool {
    literal.split('{').skip(1).any(|field| {
        field
            .split_once('}')
            .is_some_and(|(format, _)| format.contains(':') && format.ends_with('?'))
    })
}

fn is_broad_css_value_type(definition: &str) -> bool {
    let css_payload_fields =
        definition.matches("(Css").count() + definition.matches("(surgeist_css::Css").count();
    definition.contains(": CssKnownPropertyValueRef") || css_payload_fields >= 4
}

#[test]
fn scanner_ignores_bait_and_detects_renamed_structural_proxies() {
    let bait = r#"
        // trait Hidden<T: Debug> {}
        const BAIT: &str = "enum CssValue { A(CssA), B(CssB), C(CssC), D(CssD) }";
        fn visible() {}
    "#;
    let code = rust_code_without_comments_or_literals(bait);
    assert!(!code.contains("Debug"));
    assert!(!code.contains("CssValue"));
    assert!(code.contains("fn visible"));
    assert!(is_broad_css_value_type(
        "enum RenamedProxy { A(CssA), B(CssB), C(CssC), D(CssD) }"
    ));
}

#[test]
fn oracle_has_one_concrete_property_debug_payload_site() {
    let source = source(ORACLE_PATH);
    let code = rust_code_without_comments_or_literals(&source);
    let macro_span = item_span(&code, "macro_rules! assert_property_specific_value");
    let paired_arm = item_span(
        &code[macro_span.start..macro_span.end],
        "surgeist_css::CssKnownProperty::$variant,",
    );
    let paired_arm = Span {
        start: macro_span.start + paired_arm.start,
        end: macro_span.start + paired_arm.end,
    };
    let payload_sites: Vec<_> = string_literal_spans(&source)
        .into_iter()
        .filter(|span| has_debug_format_spec(&source[span.start..span.end]))
        .collect();
    assert_eq!(payload_sites.len(), 1, "oracle Debug-format payload sites");
    assert!(
        payload_sites[0].start >= paired_arm.start && payload_sites[0].end <= paired_arm.end,
        "the sole Debug payload must stay inside the paired concrete property arm"
    );
    let arm = &source[paired_arm.start..paired_arm.end];
    assert!(arm.contains("CssKnownPropertyValueRef::$variant(value)"));
    assert!(arm.contains("value.i01_subset()"));
    assert!(arm.contains("typed:{typed:?}"));
}

#[test]
fn property_value_ref_functions_are_assertion_only_and_return_unit() {
    let source = source(ORACLE_PATH);
    let code = rust_code_without_comments_or_literals(&source);
    let functions: Vec<_> = function_spans(&code)
        .into_iter()
        .filter(|span| {
            let function = &code[span.start..span.end];
            function[..function.find('{').expect("function body")]
                .contains("CssKnownPropertyValueRef")
        })
        .collect();
    assert_eq!(
        functions.len(),
        1,
        "oracle property-value-ref function count"
    );
    let function = &code[functions[0].start..functions[0].end];
    let header_end = function.find('{').expect("function body");
    let header = &function[..header_end];
    assert!(header.contains("fn assert_known_property_value"));
    assert!(
        !header.contains("->"),
        "property assertion must return unit"
    );
    assert!(
        !function.contains("String"),
        "property assertion must not collapse to text"
    );
    assert!(!function.contains("to_string"));
    assert!(!function.contains("to_owned"));
}

#[test]
fn migrated_helpers_have_no_generic_debug_or_broad_value_proxy() {
    for path in MIGRATED_HELPER_PATHS {
        let source = source(path);
        let code = rust_code_without_comments_or_literals(&source);
        let mut debug_offset = 0usize;
        while let Some(relative) = code[debug_offset..].find("Debug") {
            let offset = debug_offset + relative;
            let before = &code[..offset];
            let derive = before.rfind("derive(");
            let attribute_end = before.rfind(")]");
            assert!(
                derive.is_some_and(|derive| attribute_end.is_none_or(|end| derive > end)),
                "{path} uses Debug outside a derive attribute"
            );
            debug_offset = offset + "Debug".len();
        }
        for item in braced_type_spans(&code) {
            let definition = &code[item.start..item.end];
            assert!(
                !is_broad_css_value_type(definition),
                "{path} contains a structurally broad CSS value proxy: {}",
                definition[..definition.find('{').expect("type body")].trim()
            );
        }
        let compact = code
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        assert!(
            !compact.contains("implFrom<CssKnownPropertyValueRef")
                && !compact.contains("implTryFrom<CssKnownPropertyValueRef")
                && !compact.contains("From<&CssKnownPropertyValueRef")
                && !compact.contains("TryFrom<&CssKnownPropertyValueRef"),
            "{path} converts the heterogeneous property view"
        );
        for function in function_spans(&code) {
            let function = &code[function.start..function.end];
            let header_end = function.find('{').expect("function body");
            let header = &function[..header_end];
            if !header.contains("CssKnownPropertyValueRef") {
                continue;
            }
            let return_type = header.split_once("->").map(|(_, value)| value.trim());
            assert!(
                return_type.is_none_or(|return_type| {
                    !return_type.starts_with('(') && !return_type.contains("String")
                }),
                "{path} collapses a property value ref into a tuple or owned string"
            );
            assert!(
                !function.contains("to_string")
                    && !function.contains("to_owned")
                    && !function.contains("format!"),
                "{path} contains a reusable property-to-payload conversion"
            );
        }
    }
}
