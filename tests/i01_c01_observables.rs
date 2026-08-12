use std::collections::BTreeSet;

use surgeist_css::{
    CssDeclaration, CssDeclarationContextRef, CssErrorCode, CssImportance, CssPropertyNameRef,
    CssRecoveryAction, CssRecoveryDiagnostic, CssRule, CssScopedRule, ErrorKind, parse_sheet,
    parse_style_attribute,
};

#[path = "catalog_inventory/vectors.rs"]
mod catalog_vectors;
#[path = "i01_c01_observables/cases.rs"]
mod i01_cases;

use i01_cases::{Case, EntryPoint, FeatureMode, focused_cases, non_property_cases};

const FIXTURE: &str = include_str!("fixtures/i01-c01-observables.tsv");
const HEADER: &str = "case_id\towner\tentry\tfeature\tinput\tclean\tretained\tvalues\tdiagnostics";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    case_id: String,
    owner: String,
    entry: String,
    feature: String,
    input: String,
    clean: String,
    retained: String,
    values: String,
    diagnostics: String,
}

impl Row {
    fn fields(&self) -> [&str; 9] {
        [
            &self.case_id,
            &self.owner,
            &self.entry,
            &self.feature,
            &self.input,
            &self.clean,
            &self.retained,
            &self.values,
            &self.diagnostics,
        ]
    }
}

fn parse_fixture(source: &str) -> Result<Vec<Row>, String> {
    let mut lines = source.lines();
    let header = lines.next().ok_or("fixture is empty")?;
    if header != HEADER {
        return Err(format!("unknown or reordered columns: `{header}`"));
    }

    let mut rows: Vec<Row> = Vec::new();
    let mut ids = BTreeSet::new();
    for (line_index, line) in lines.enumerate() {
        if line.is_empty() {
            return Err(format!("blank row at fixture line {}", line_index + 2));
        }
        let raw = line.split('\t').collect::<Vec<_>>();
        if raw.len() != 9 {
            return Err(format!(
                "fixture line {} has {} columns, expected 9",
                line_index + 2,
                raw.len()
            ));
        }
        let fields = raw
            .into_iter()
            .map(unescape)
            .collect::<Result<Vec<_>, _>>()?;
        if fields
            .iter()
            .enumerate()
            .any(|(index, field)| index != 4 && field.is_empty())
        {
            return Err(format!(
                "absent required observable at fixture line {}",
                line_index + 2
            ));
        }
        let row = Row {
            case_id: fields[0].clone(),
            owner: fields[1].clone(),
            entry: fields[2].clone(),
            feature: fields[3].clone(),
            input: fields[4].clone(),
            clean: fields[5].clone(),
            retained: fields[6].clone(),
            values: fields[7].clone(),
            diagnostics: fields[8].clone(),
        };
        if !ids.insert(row.case_id.clone()) {
            return Err(format!("duplicate case ID `{}`", row.case_id));
        }
        if let Some(previous) = rows.last()
            && previous.case_id >= row.case_id
        {
            return Err(format!(
                "noncanonical case order: `{}` before `{}`",
                previous.case_id, row.case_id
            ));
        }
        match row.entry.as_str() {
            "sheet" | "style" => {}
            value => return Err(format!("{}: unknown entry point `{value}`", row.case_id)),
        }
        match row.feature.as_str() {
            "both" | "default" | "app-strict" => {}
            value => return Err(format!("{}: unknown feature mode `{value}`", row.case_id)),
        }
        match row.clean.as_str() {
            "true" | "false" => {}
            value => return Err(format!("{}: invalid clean state `{value}`", row.case_id)),
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("fixture has no cases".to_owned());
    }
    Ok(rows)
}

fn unescape(field: &str) -> Result<String, String> {
    let mut output = String::new();
    let mut chars = field.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match chars.next() {
            Some('\\') => output.push('\\'),
            Some('t') => output.push('\t'),
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some(escaped) => return Err(format!("malformed escape `\\{escaped}`")),
            None => return Err("trailing fixture escape".to_owned()),
        }
    }
    Ok(output)
}

fn expected_cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for vector in catalog_vectors::PROPERTY_POSITIVE_VECTORS {
        cases.push(Case::new(
            format!("catalog.property.{}.positive", vector.id),
            format!("catalog_inventory::{}/positive", vector.id),
            EntryPoint::Style,
            FeatureMode::Both,
            format!("{}: {}", vector.canonical_name, vector.authored_value),
        ));
    }
    for vector in catalog_vectors::PROPERTY_NEGATIVE_VECTORS {
        cases.push(Case::new(
            format!("catalog.property.{}.boundary", vector.id),
            format!("catalog_inventory::{}/negative", vector.id),
            EntryPoint::Style,
            FeatureMode::Both,
            format!("{}: {}", vector.canonical_name, vector.authored_value),
        ));
    }
    for vector in catalog_vectors::PROPERTY_POSITIVE_VECTORS {
        cases.push(Case::new(
            format!("focused.property-schema.{}.ordinary", vector.id),
            format!("property_schema::property_schema_parser_identity_matches_every_frozen_name/{}/ordinary", vector.id),
            EntryPoint::Sheet,
            FeatureMode::Both,
            format!(".test {{ {}: {}; }}", vector.canonical_name.to_ascii_uppercase(), vector.authored_value),
        ));
        cases.push(Case::new(
            format!("focused.property-schema.{}.important", vector.id),
            format!("property_schema::property_schema_parser_identity_matches_every_frozen_name/{}/important", vector.id),
            EntryPoint::Sheet,
            FeatureMode::Both,
            format!(".test {{ {}: {} !important; }}", vector.canonical_name.to_ascii_uppercase(), if vector.canonical_name == "all" { "inherit" } else { vector.authored_value }),
        ));
    }
    cases.extend(non_property_cases());
    cases.extend(focused_cases());
    cases.sort_by(|left, right| left.id.cmp(&right.id));
    cases
}

#[test]
fn i01_observable_fixture_has_the_exact_independent_case_union() {
    let rows = parse_fixture(FIXTURE).expect("valid I01 observable fixture");
    let expected = expected_cases();
    let expected_ids = expected
        .iter()
        .map(|case| case.id.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(expected_ids.len(), expected.len(), "manifest ID collision");
    assert_eq!(
        catalog_vectors::PROPERTY_POSITIVE_VECTORS.len(),
        179,
        "imported positive catalog-vector count changed"
    );
    assert_eq!(
        catalog_vectors::PROPERTY_NEGATIVE_VECTORS.len(),
        179,
        "imported boundary catalog-vector count changed"
    );
    assert_eq!(rows.len(), expected.len(), "exact I01 case union size");
    for (row, case) in rows.iter().zip(expected.iter()) {
        assert_eq!(row.case_id, case.id, "case-union identity");
        assert_eq!(row.owner, case.owner, "{} owner", case.id);
        assert_eq!(row.entry, case.entry.as_str(), "{} entry", case.id);
        assert_eq!(row.feature, case.feature.as_str(), "{} feature", case.id);
        assert_eq!(row.input, case.input, "{} authored input", case.id);
    }
}

#[test]
fn i01_observable_fixture_matches_every_public_report() {
    let rows = parse_fixture(FIXTURE).expect("valid I01 observable fixture");
    for row in rows {
        if row.feature == "app-strict" && !cfg!(feature = "app-strict") {
            continue;
        }
        let actual = observe(&row);
        assert_eq!(
            actual.fields(),
            row.fields(),
            "{} public report",
            row.case_id
        );
        #[cfg(feature = "app-strict")]
        assert_strict_parity(&row);
    }
}

#[test]
fn i01_observable_reader_rejects_union_and_observable_mutations() {
    let rows = parse_fixture(FIXTURE).expect("valid fixture");
    assert!(
        parse_fixture(&FIXTURE.replacen(HEADER, &format!("{HEADER}\textra"), 1))
            .expect_err("unknown column must fail")
            .contains("unknown or reordered columns")
    );
    let duplicate = format!(
        "{HEADER}\n{}\n{}\n",
        render_row(&rows[0]),
        render_row(&rows[0])
    );
    assert!(
        parse_fixture(&duplicate)
            .expect_err("duplicate ID must fail")
            .contains(&rows[0].case_id)
    );
    let malformed = format!("{HEADER}\n{}\\q\n", render_row(&rows[0]));
    assert!(
        parse_fixture(&malformed)
            .expect_err("malformed escape must fail")
            .contains("malformed escape")
    );
    let noncanonical = format!(
        "{HEADER}\n{}\n{}\n",
        render_row(&rows[1]),
        render_row(&rows[0])
    );
    assert!(
        parse_fixture(&noncanonical)
            .expect_err("noncanonical order must fail")
            .contains("noncanonical case order")
    );
    let removed = rows
        .iter()
        .skip(1)
        .map(render_row)
        .collect::<Vec<_>>()
        .join("\n");
    let removed = format!("{HEADER}\n{removed}\n");
    let removed_rows = parse_fixture(&removed).expect("well-formed incomplete fixture");
    assert_ne!(
        removed_rows.len(),
        expected_cases().len(),
        "removed case must fail exact union"
    );

    let mut missing = rows.clone();
    missing[0].retained.clear();
    let missing = format!(
        "{HEADER}\n{}\n",
        missing
            .iter()
            .map(render_row)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        parse_fixture(&missing)
            .expect_err("missing observable must fail")
            .contains("absent required observable"),
        "responsible case: {}",
        rows[0].case_id
    );

    let repeated_index = rows
        .iter()
        .position(|row| row.diagnostics.split('~').count() > 1)
        .expect("fixture must contain repeated diagnostics");
    let mut repeated = rows.clone();
    repeated[repeated_index].diagnostics = repeated[repeated_index]
        .diagnostics
        .split_once('~')
        .expect("repeated diagnostic")
        .0
        .to_owned();
    assert_ne!(
        observe(&repeated[repeated_index]).diagnostics,
        repeated[repeated_index].diagnostics,
        "{} repeated diagnostic removal must fail",
        repeated[repeated_index].case_id
    );
}

fn render_row(row: &Row) -> String {
    row.fields().map(escape).join("\t")
}

fn escape(field: &str) -> String {
    field
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn observe(expected: &Row) -> Row {
    let (clean, retained, values, diagnostics) = match expected.entry.as_str() {
        "sheet" => {
            let report = parse_sheet(&expected.input);
            let (retained, values) = sheet_observables(report.syntax().rules());
            (
                report.is_clean(),
                retained,
                values,
                diagnostics_observable(report.diagnostics()),
            )
        }
        "style" => {
            let report = parse_style_attribute(&expected.input);
            let (retained, values) = declaration_observables(report.syntax().as_slice());
            (
                report.is_clean(),
                retained,
                values,
                diagnostics_observable(report.diagnostics()),
            )
        }
        _ => unreachable!("validated fixture entry"),
    };
    Row {
        case_id: expected.case_id.clone(),
        owner: expected.owner.clone(),
        entry: expected.entry.clone(),
        feature: expected.feature.clone(),
        input: expected.input.clone(),
        clean: clean.to_string(),
        retained: nonempty(retained.join("~")),
        values: nonempty(values.join("~")),
        diagnostics: nonempty(diagnostics.join("~")),
    }
}

fn nonempty(value: String) -> String {
    if value.is_empty() {
        "-".to_owned()
    } else {
        value
    }
}

fn sheet_observables(rules: &[CssRule]) -> (Vec<String>, Vec<String>) {
    let mut retained = Vec::new();
    let mut values = Vec::new();
    for rule in rules {
        rule_observables(rule, &mut retained, &mut values);
    }
    (retained, values)
}

fn rule_observables(rule: &CssRule, retained: &mut Vec<String>, values: &mut Vec<String>) {
    match rule {
        CssRule::Import(_) => retained.push("rule:baseline.rule.import".to_owned()),
        CssRule::LayerStatement(_) => {
            retained.push("rule:baseline.rule.layer-statement".to_owned())
        }
        CssRule::LayerBlock(rule) => {
            retained.push("rule:baseline.rule.layer-block".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, values);
            }
        }
        CssRule::FontFace(_) => retained.push("rule:baseline.rule.font-face".to_owned()),
        CssRule::Keyframes(rule) => {
            retained.push("rule:baseline.rule.keyframes".to_owned());
            for block in rule.blocks() {
                for declaration in block.declarations().iter() {
                    let id = match declaration.property_name() {
                        CssPropertyNameRef::Known(property) => property.stable_id().to_owned(),
                        CssPropertyNameRef::Custom(name) => format!("custom:{}", name.as_str()),
                        _ => "future-property".to_owned(),
                    };
                    retained.push(format!("property:{id}"));
                }
            }
        }
        CssRule::Style(rule) => {
            retained.push("rule:baseline.rule.style".to_owned());
            let (ids, authored) = declaration_observables(rule.declarations().as_slice());
            retained.extend(ids);
            values.extend(authored);
        }
        CssRule::Media(rule) => {
            retained.push("rule:baseline.rule.media".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, values);
            }
        }
        CssRule::Container(rule) => {
            retained.push("rule:baseline.rule.container".to_owned());
            for child in rule.rules() {
                rule_observables(child, retained, values);
            }
        }
        CssRule::Scope(rule) => {
            retained.push("rule:baseline.rule.scope".to_owned());
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values);
            }
        }
        _ => retained.push("rule:future".to_owned()),
    }
}

fn scoped_rule_observables(
    rule: &CssScopedRule,
    retained: &mut Vec<String>,
    values: &mut Vec<String>,
) {
    match rule {
        CssScopedRule::Style(rule) => {
            retained.push("rule:baseline.rule.style".to_owned());
            let (ids, authored) = declaration_observables(rule.declarations().as_slice());
            retained.extend(ids);
            values.extend(authored);
        }
        CssScopedRule::Media(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values);
            }
        }
        CssScopedRule::Container(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values);
            }
        }
        CssScopedRule::LayerStatement(_) => {
            retained.push("rule:baseline.rule.layer-statement".to_owned())
        }
        CssScopedRule::LayerBlock(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values);
            }
        }
        CssScopedRule::Scope(rule) => {
            for child in rule.rules().rules() {
                scoped_rule_observables(child, retained, values);
            }
        }
    }
}

fn declaration_observables(declarations: &[CssDeclaration]) -> (Vec<String>, Vec<String>) {
    let mut retained = Vec::new();
    let mut values = Vec::new();
    declaration_values(
        declarations
            .iter()
            .map(|declaration| (declaration.property_name(), Some(declaration))),
        &mut retained,
        &mut values,
    );
    (retained, values)
}

fn declaration_values<'a>(
    declarations: impl Iterator<Item = (CssPropertyNameRef<'a>, Option<&'a CssDeclaration>)>,
    retained: &mut Vec<String>,
    values: &mut Vec<String>,
) {
    // At the I01 base, exact authored text is publicly observable only for custom and
    // substitution-dependent values. Typed/global rows therefore freeze their complete authored
    // source in the fixture's `input` field and their currently public semantic payload here.
    // T3 must adapt this reader to compare wrapper `as_css()` against that already-frozen source
    // slice; it must not rewrite the TSV oracle.
    for (name, declaration) in declarations {
        let id = match name {
            CssPropertyNameRef::Known(property) => property.stable_id().to_owned(),
            CssPropertyNameRef::Custom(name) => format!("custom:{}", name.as_str()),
            _ => "future-property".to_owned(),
        };
        retained.push(format!("property:{id}"));
        if let Some(declaration) = declaration {
            let importance = match declaration.importance() {
                CssImportance::Normal => "normal",
                CssImportance::Important => "important",
            };
            if let Some(custom) = declaration.custom() {
                let authored = custom.value().value().map_or_else(
                    || format!("global:{:?}", custom.value().global()),
                    |value| value.as_css().to_owned(),
                );
                values.push(format!("{id}={authored}@{importance}"));
            } else if let Some(known) = declaration.known() {
                values.push(format!("{id}={}@{importance}", known_value(known)));
            }
        }
    }
}

fn declared_value<T: std::fmt::Debug>(value: &surgeist_css::CssDeclaredValue<T>) -> String {
    match value {
        surgeist_css::CssDeclaredValue::Value(value) => format!("typed:{value:?}"),
        surgeist_css::CssDeclaredValue::Global(value) => format!("global:{value:?}"),
        surgeist_css::CssDeclaredValue::SubstitutionDependent(value) => {
            format!("substitution:{}", value.as_css())
        }
        _ => "future".to_owned(),
    }
}

fn known_value(known: &surgeist_css::CssKnownDeclaration) -> String {
    macro_rules! arms {
        ($($variant:ident),+ $(,)?) => { match known {
            $(surgeist_css::CssKnownDeclaration::$variant(value) => declared_value(value),)+
            _ => "future".to_owned(),
        } };
    }
    match known {
        surgeist_css::CssKnownDeclaration::All(value) => match value {
            surgeist_css::CssAllDeclaredValue::Global(value) => format!("global:{value:?}"),
            surgeist_css::CssAllDeclaredValue::SubstitutionDependent(value) => {
                format!("substitution:{}", value.as_css())
            }
            _ => "future".to_owned(),
        },
        _ => arms!(
            Display,
            BoxSizing,
            Position,
            Direction,
            Overflow,
            OverflowX,
            OverflowY,
            FlexDirection,
            FlexWrap,
            Float,
            Clear,
            AlignContent,
            JustifyContent,
            AlignItems,
            AlignSelf,
            JustifyItems,
            JustifySelf,
            PlaceContent,
            PlaceItems,
            PlaceSelf,
            Visibility,
            Content,
            ContentVisibility,
            ListStyleType,
            ListStylePosition,
            ListStyleImage,
            ListStyle,
            CounterReset,
            CounterIncrement,
            CounterSet,
            Width,
            Height,
            MinWidth,
            MinHeight,
            MaxWidth,
            MaxHeight,
            FlexBasis,
            Gap,
            RowGap,
            ColumnGap,
            GridFlowTolerance,
            GridTemplateRows,
            GridTemplateColumns,
            GridTemplateAreas,
            GridTemplate,
            GridAutoRows,
            GridAutoColumns,
            GridAutoFlow,
            GridRowStart,
            GridRowEnd,
            GridColumnStart,
            GridColumnEnd,
            GridRow,
            GridColumn,
            GridArea,
            Grid,
            FontSize,
            LineHeight,
            WritingMode,
            TextAlign,
            TextAlignLast,
            TextIndent,
            VerticalAlign,
            FontFamily,
            Font,
            FontWeight,
            FontStyle,
            FontStretch,
            FontVariant,
            FontFeatureSettings,
            LetterSpacing,
            TextWrap,
            WhiteSpace,
            WordBreak,
            OverflowWrap,
            TextOverflow,
            TextDecoration,
            TextDecorationLine,
            TextDecorationColor,
            TextDecorationStyle,
            TextDecorationThickness,
            TextTransform,
            Inset,
            Top,
            Right,
            Bottom,
            Left,
            ZIndex,
            BoxDecorationBreak,
            Margin,
            MarginTop,
            MarginRight,
            MarginBottom,
            MarginLeft,
            Padding,
            PaddingTop,
            PaddingRight,
            PaddingBottom,
            PaddingLeft,
            Border,
            BorderTop,
            BorderRight,
            BorderBottom,
            BorderLeft,
            BorderWidth,
            BorderTopWidth,
            BorderRightWidth,
            BorderBottomWidth,
            BorderLeftWidth,
            Color,
            Background,
            BackgroundColor,
            BorderColor,
            BorderTopColor,
            BorderRightColor,
            BorderBottomColor,
            BorderLeftColor,
            BackgroundImage,
            BackgroundPosition,
            BackgroundSize,
            BackgroundRepeat,
            BackgroundOrigin,
            BackgroundClip,
            BackgroundAttachment,
            BorderStyle,
            BorderTopStyle,
            BorderRightStyle,
            BorderBottomStyle,
            BorderLeftStyle,
            BorderRadius,
            BorderTopLeftRadius,
            BorderTopRightRadius,
            BorderBottomRightRadius,
            BorderBottomLeftRadius,
            BoxShadow,
            Opacity,
            FlexGrow,
            FlexShrink,
            Order,
            Flex,
            JustifyTracks,
            AlignTracks,
            AspectRatio,
            ScrollbarWidth,
            Cursor,
            PointerEvents,
            UserSelect,
            Outline,
            OutlineColor,
            OutlineStyle,
            OutlineWidth,
            Transform,
            TransformOrigin,
            Translate,
            Rotate,
            Scale,
            Filter,
            BackdropFilter,
            ClipPath,
            Mask,
            MaskImage,
            MaskSize,
            MaskPosition,
            MaskRepeat,
            TransitionProperty,
            TransitionDuration,
            TransitionDelay,
            TransitionTimingFunction,
            Transition,
            AnimationName,
            AnimationDuration,
            AnimationDelay,
            AnimationTimingFunction,
            AnimationIterationCount,
            AnimationDirection,
            AnimationFillMode,
            AnimationPlayState,
            Animation,
        ),
    }
}

fn diagnostics_observable(diagnostics: &[CssRecoveryDiagnostic]) -> Vec<String> {
    diagnostics.iter().map(diagnostic_observable).collect()
}

fn diagnostic_observable(diagnostic: &CssRecoveryDiagnostic) -> String {
    let error = diagnostic.error();
    let position = error.position();
    let span = diagnostic.span();
    format!(
        "{}/{}/{}@{}:{}:{}>{}:{}:{}-{}:{}:{}:{}",
        code_name(error.code()),
        root_and_payload(error.kind()),
        action_name(diagnostic.action()),
        position.byte_offset().value(),
        position.line().value(),
        position.column().value(),
        span.start().byte_offset().value(),
        span.start().line().value(),
        span.start().column().value(),
        span.end().byte_offset().value(),
        span.end().line().value(),
        span.end().column().value(),
        span.end().byte_offset().value(),
    )
}

fn token(token: Option<&surgeist_css::CssTokenSummary>) -> String {
    token.map_or_else(
        || "-".to_owned(),
        |token| format!("{:?}:{}", token.kind(), token.authored()),
    )
}

fn root_and_payload(kind: &ErrorKind) -> String {
    match kind {
        ErrorKind::UnexpectedEnd(detail) => {
            format!("UnexpectedEnd:{}", detail.expectation().as_str())
        }
        ErrorKind::UnexpectedToken(detail) => format!(
            "UnexpectedToken:{}:{}",
            detail.expectation().as_str(),
            token(Some(detail.encountered()))
        ),
        ErrorKind::InvalidEncodingDeclaration(detail) => format!(
            "InvalidEncodingDeclaration:{}:{}",
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidAtRulePlacement(detail) => format!(
            "InvalidAtRulePlacement:{}:{}",
            detail.name().as_str(),
            detail.expected_context().as_str()
        ),
        ErrorKind::InvalidAtRulePrelude(detail) => format!(
            "InvalidAtRulePrelude:{}:{}:{}:{}",
            detail.name().as_str(),
            detail.production().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidAtRuleBody(detail) => format!(
            "InvalidAtRuleBody:{}:{}:{}:{}",
            detail.name().as_str(),
            detail.production().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::UnknownAtRule(detail) => format!("UnknownAtRule:{}", detail.name().as_str()),
        ErrorKind::UnsupportedAtRule(detail) => format!(
            "UnsupportedAtRule:{}:{}",
            detail.name().as_str(),
            detail.feature().as_str()
        ),
        ErrorKind::InvalidQualifiedRule(detail) => format!(
            "InvalidQualifiedRule:{}:{}:{}",
            detail.production().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidSelector(detail) => format!(
            "InvalidSelector:{}:{}:{}",
            detail.production().map_or("-", |value| value.as_str()),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidMediaQuery(detail) => format!(
            "InvalidMediaQuery:{}:{}:{}",
            detail.feature().map_or("-", |value| value.as_str()),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::UnknownProperty(detail) => format!("UnknownProperty:{}", detail.name().as_str()),
        ErrorKind::UnsupportedProperty(detail) => format!(
            "UnsupportedProperty:{}:{}",
            detail.name().as_str(),
            detail.feature().as_str()
        ),
        ErrorKind::InvalidPropertyValue(detail) => format!(
            "InvalidPropertyValue:{}:{}:{}",
            detail.property().stable_id(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidDeclarationAnnotation(detail) => format!(
            "InvalidDeclarationAnnotation:{}:{}",
            declaration_context(detail.context()),
            token(Some(detail.encountered()))
        ),
        ErrorKind::UnknownDescriptor(detail) => format!(
            "UnknownDescriptor:{}:{}",
            detail.at_rule().as_str(),
            detail.descriptor().as_str()
        ),
        ErrorKind::UnsupportedDescriptor(detail) => format!(
            "UnsupportedDescriptor:{}:{}:{}",
            detail.at_rule().as_str(),
            detail.descriptor().as_str(),
            detail.feature().as_str()
        ),
        ErrorKind::InvalidDescriptorValue(detail) => format!(
            "InvalidDescriptorValue:{}:{}:{}:{}",
            detail.at_rule().as_str(),
            detail.descriptor().as_str(),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::InvalidDescriptorCombination(detail) => format!(
            "InvalidDescriptorCombination:{}:{}:{}",
            detail.at_rule().as_str(),
            detail.responsible().as_str(),
            detail
                .conflicting()
                .iter()
                .map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
        ErrorKind::InvalidColorSyntax(detail) => format!(
            "InvalidColorSyntax:{}:{}:{}",
            detail.component().map_or("-", |value| value.as_str()),
            detail.expectation().as_str(),
            token(detail.encountered())
        ),
        ErrorKind::NestingLimit(detail) => format!(
            "NestingLimit:{}:{}",
            detail.limit(),
            detail.enclosing_production().as_str()
        ),
        _ => "Future".to_owned(),
    }
}

fn declaration_context(context: CssDeclarationContextRef<'_>) -> String {
    match context {
        CssDeclarationContextRef::KnownProperty(property) => {
            format!("known:{}", property.stable_id())
        }
        CssDeclarationContextRef::CustomProperty(name) => format!("custom:{}", name.as_str()),
        CssDeclarationContextRef::Keyframe(property) => {
            format!("keyframe:{}", property.stable_id())
        }
        CssDeclarationContextRef::KeyframeCustomProperty(name) => {
            format!("keyframe-custom:{}", name.as_str())
        }
        CssDeclarationContextRef::Descriptor {
            at_rule,
            descriptor,
        } => format!("descriptor:{}:{}", at_rule.as_str(), descriptor.as_str()),
        _ => "future".to_owned(),
    }
}

fn code_name(code: CssErrorCode) -> &'static str {
    match code {
        CssErrorCode::UnexpectedEnd => "UnexpectedEnd",
        CssErrorCode::UnexpectedToken => "UnexpectedToken",
        CssErrorCode::InvalidEncodingDeclaration => "InvalidEncodingDeclaration",
        CssErrorCode::InvalidAtRulePlacement => "InvalidAtRulePlacement",
        CssErrorCode::InvalidAtRulePrelude => "InvalidAtRulePrelude",
        CssErrorCode::InvalidAtRuleBody => "InvalidAtRuleBody",
        CssErrorCode::UnknownAtRule => "UnknownAtRule",
        CssErrorCode::UnsupportedAtRule => "UnsupportedAtRule",
        CssErrorCode::InvalidQualifiedRule => "InvalidQualifiedRule",
        CssErrorCode::InvalidSelector => "InvalidSelector",
        CssErrorCode::InvalidMediaQuery => "InvalidMediaQuery",
        CssErrorCode::UnknownProperty => "UnknownProperty",
        CssErrorCode::UnsupportedProperty => "UnsupportedProperty",
        CssErrorCode::InvalidPropertyValue => "InvalidPropertyValue",
        CssErrorCode::InvalidDeclarationAnnotation => "InvalidDeclarationAnnotation",
        CssErrorCode::UnknownDescriptor => "UnknownDescriptor",
        CssErrorCode::UnsupportedDescriptor => "UnsupportedDescriptor",
        CssErrorCode::InvalidDescriptorValue => "InvalidDescriptorValue",
        CssErrorCode::InvalidDescriptorCombination => "InvalidDescriptorCombination",
        CssErrorCode::InvalidColorSyntax => "InvalidColorSyntax",
        CssErrorCode::NestingLimit => "NestingLimit",
        _ => "Future",
    }
}

fn action_name(action: CssRecoveryAction) -> &'static str {
    match action {
        CssRecoveryAction::DropDeclaration => "DropDeclaration",
        CssRecoveryAction::DropDescriptor => "DropDescriptor",
        CssRecoveryAction::DropQualifiedRule => "DropQualifiedRule",
        CssRecoveryAction::DropAtRule => "DropAtRule",
        CssRecoveryAction::DropKeyframeBlock => "DropKeyframeBlock",
        CssRecoveryAction::DropSelectorListItem => "DropSelectorListItem",
        CssRecoveryAction::ReplaceMediaQueryWithNever => "ReplaceMediaQueryWithNever",
        CssRecoveryAction::RetainWithImplicitClosure => "RetainWithImplicitClosure",
        CssRecoveryAction::IgnoreLegacyToken => "IgnoreLegacyToken",
        CssRecoveryAction::StopAtNestingLimit => "StopAtNestingLimit",
        _ => "Future",
    }
}

#[cfg(feature = "app-strict")]
fn assert_strict_parity(row: &Row) {
    match row.entry.as_str() {
        "sheet" => {
            let ordinary = parse_sheet(&row.input);
            let strict = surgeist_css::validate_sheet(&row.input);
            if ordinary.is_clean() {
                assert_eq!(
                    strict,
                    Ok(ordinary.syntax().clone()),
                    "{} strict sheet",
                    row.case_id
                );
            } else {
                assert_eq!(
                    strict.expect_err("recovered sheet").diagnostics(),
                    ordinary.diagnostics(),
                    "{} strict sheet",
                    row.case_id
                );
            }
        }
        "style" => {
            let ordinary = parse_style_attribute(&row.input);
            let strict = surgeist_css::validate_style_attribute(&row.input);
            if ordinary.is_clean() {
                assert_eq!(
                    strict,
                    Ok(ordinary.syntax().clone()),
                    "{} strict style",
                    row.case_id
                );
            } else {
                assert_eq!(
                    strict.expect_err("recovered style").diagnostics(),
                    ordinary.diagnostics(),
                    "{} strict style",
                    row.case_id
                );
            }
        }
        _ => unreachable!(),
    }
}
