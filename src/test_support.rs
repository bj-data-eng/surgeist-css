use crate::validation::{SupportedProperty, supported_properties};
use crate::{
    CssDeclaration, CssGlobalKeyword, CssProperty, CssRule, CssSheet, CssStyleRule, CssValue,
    Error, ErrorKind, parse_sheet,
};

pub(crate) struct AcceptedDeclarationCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_property: CssProperty,
    pub(crate) expected_value: CssValue,
}

impl AcceptedDeclarationCase {
    pub(crate) fn supported_global_inherit(supported_property: &SupportedProperty) -> Self {
        Self::global_inherit(supported_property.name, supported_property.property.clone())
    }

    pub(crate) fn global_inherit(
        property_name: &'static str,
        expected_property: CssProperty,
    ) -> Self {
        Self {
            label: property_name,
            property_name,
            authored_value: "inherit",
            expected_property,
            expected_value: CssValue::GlobalKeyword(CssGlobalKeyword::Inherit),
        }
    }

    pub(crate) fn assert_accepts(&self) -> CssDeclaration {
        let declaration = parse_single_declaration(self.property_name, self.authored_value);
        assert_eq!(
            declaration.property(),
            &self.expected_property,
            "{} parsed to the wrong property",
            self.label,
        );
        assert_eq!(
            declaration.value(),
            &self.expected_value,
            "{} parsed to the wrong value",
            self.label,
        );
        declaration
    }
}

pub(crate) struct AcceptedValueCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_property: CssProperty,
    pub(crate) assert_value: fn(&CssValue),
}

impl AcceptedValueCase {
    pub(crate) fn assert_accepts(&self) -> CssDeclaration {
        let declaration = parse_single_declaration(self.property_name, self.authored_value);
        assert_eq!(
            declaration.property(),
            &self.expected_property,
            "{} parsed to the wrong property",
            self.label,
        );
        (self.assert_value)(declaration.value());
        declaration
    }
}

pub(crate) enum ExpectedErrorKind {
    InvalidSyntax,
    InvalidSelector,
    InvalidSyntaxOrUnsupportedValueForProperty { property: &'static str },
    UnsupportedAtRule { name: &'static str },
    UnknownProperty { name: &'static str },
    UnsupportedValueForProperty { property: &'static str },
    UnsupportedValue { property: Option<&'static str> },
}

impl ExpectedErrorKind {
    fn assert_matches(&self, actual: &ErrorKind, label: &str) {
        match (self, actual) {
            (
                Self::InvalidSyntax,
                ErrorKind::UnexpectedEnd(_)
                | ErrorKind::UnexpectedToken(_)
                | ErrorKind::InvalidQualifiedRule(_)
                | ErrorKind::InvalidAtRulePrelude(_)
                | ErrorKind::InvalidAtRuleBody(_)
                | ErrorKind::InvalidPropertyValue(_),
            ) => {}
            (Self::InvalidSelector, ErrorKind::InvalidSelector(_)) => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { property },
                ErrorKind::InvalidPropertyValue(detail),
            ) if crate::validation::property_for_supported_name(property).as_ref()
                == Some(detail.property()) => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { .. },
                ErrorKind::UnexpectedEnd(_)
                | ErrorKind::UnexpectedToken(_)
                | ErrorKind::InvalidQualifiedRule(_)
                | ErrorKind::InvalidColorSyntax(_),
            ) => {}
            (Self::UnsupportedAtRule { name }, ErrorKind::UnsupportedAtRule(detail))
                if *name == detail.name().as_str() => {}
            (Self::UnsupportedAtRule { name }, ErrorKind::UnknownAtRule(detail))
                if *name == detail.name().as_str() => {}
            (Self::UnknownProperty { name }, ErrorKind::UnknownProperty(detail))
                if *name == detail.name().as_str() => {}
            (
                Self::UnsupportedValueForProperty { property },
                ErrorKind::InvalidPropertyValue(detail),
            ) if crate::validation::property_for_supported_name(property).as_ref()
                == Some(detail.property()) => {}
            (Self::UnsupportedValueForProperty { .. }, ErrorKind::InvalidColorSyntax(_)) => {}
            (
                Self::UnsupportedValue {
                    property: Some(property),
                },
                ErrorKind::InvalidPropertyValue(detail),
            ) if crate::validation::property_for_supported_name(property).as_ref()
                == Some(detail.property()) => {}
            (
                Self::UnsupportedValue { property: None },
                ErrorKind::UnexpectedEnd(_)
                | ErrorKind::UnexpectedToken(_)
                | ErrorKind::InvalidQualifiedRule(_)
                | ErrorKind::InvalidColorSyntax(_)
                | ErrorKind::InvalidMediaQuery(_),
            ) => {}
            _ => panic!("{label} rejected with unexpected error kind: {actual:?}"),
        }
    }
}

pub(crate) struct RejectedSheetCase {
    pub(crate) label: &'static str,
    pub(crate) input: &'static str,
    pub(crate) expected_error: ExpectedErrorKind,
}

impl RejectedSheetCase {
    pub(crate) fn assert_rejects(&self) -> Error {
        let error = parse_sheet(self.input).expect_err("invalid CSS must reject the whole sheet");
        self.expected_error.assert_matches(error.kind(), self.label);
        error
    }
}

pub(crate) struct RejectedDeclarationCase {
    pub(crate) label: &'static str,
    pub(crate) property_name: &'static str,
    pub(crate) authored_value: &'static str,
    pub(crate) expected_error: ExpectedErrorKind,
    pub(crate) property_name_should_be_recognized: bool,
}

impl RejectedDeclarationCase {
    pub(crate) fn assert_rejects(&self) -> Error {
        let css = declaration_sheet(self.property_name, self.authored_value);
        let error = assert_sheet_rejected(&css, &self.expected_error);
        assert_eq!(
            matches!(error.kind(), ErrorKind::UnknownProperty(_)),
            !self.property_name_should_be_recognized,
            "{} property-name recognition mismatch",
            self.label,
        );
        error
    }
}

pub(crate) fn assert_accepts_declarations(cases: &[AcceptedDeclarationCase]) {
    for case in cases {
        case.assert_accepts();
    }
}

pub(crate) fn assert_accepts_value_cases(cases: &[AcceptedValueCase]) {
    for case in cases {
        case.assert_accepts();
    }
}

pub(crate) fn assert_rejects_declarations(cases: &[RejectedDeclarationCase]) {
    for case in cases {
        case.assert_rejects();
    }
}

pub(crate) fn assert_rejects_sheets(cases: &[RejectedSheetCase]) {
    for case in cases {
        case.assert_rejects();
    }
}

pub(crate) fn parse_single_declaration(
    property_name: &str,
    authored_value: &str,
) -> CssDeclaration {
    parse_single_declaration_from_sheet(&declaration_sheet(property_name, authored_value))
}

pub(crate) fn parse_single_declaration_from_sheet(input: &str) -> CssDeclaration {
    let sheet = parse_sheet(input).unwrap_or_else(|error| panic!("{input} should parse: {error}"));
    only_declaration(&sheet, input)
}

pub(crate) fn parse_single_declaration_value(
    property_name: &str,
    authored_value: &str,
) -> CssValue {
    parse_single_declaration(property_name, authored_value)
        .value()
        .clone()
}

pub(crate) fn assert_sheet_rejected(input: &str, expected_error: &ExpectedErrorKind) -> Error {
    let error = parse_sheet(input).expect_err("invalid CSS must reject the whole sheet");
    expected_error.assert_matches(error.kind(), input);
    error
}

pub(crate) fn accepted_declaration_cases() -> Vec<AcceptedDeclarationCase> {
    supported_properties()
        .iter()
        .map(AcceptedDeclarationCase::supported_global_inherit)
        .collect()
}

fn declaration_sheet(property_name: &str, authored_value: &str) -> String {
    format!(".test {{ {property_name}: {authored_value}; }}")
}

fn only_declaration(sheet: &CssSheet, input: &str) -> CssDeclaration {
    let [rule] = sheet.rules() else {
        panic!("{input} should parse exactly one rule");
    };
    let rule = style_rule(rule);
    let [declaration] = rule.declarations() else {
        panic!("{input} should parse exactly one declaration");
    };
    declaration.clone()
}

fn style_rule(rule: &CssRule) -> &CssStyleRule {
    match rule {
        CssRule::Style(rule) => rule,
        unexpected => panic!("expected style rule, got {unexpected:?}"),
    }
}
