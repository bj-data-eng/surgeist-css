use crate::validation::{SupportedProperty, supported_properties};
use crate::{
    CssDeclaration, CssGlobalKeyword, CssProperty, CssSheet, CssValue, Error, ErrorKind,
    parse_sheet,
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
    InvalidSyntaxOrUnsupportedValueForProperty {
        property: &'static str,
    },
    UnsupportedAtRule {
        name: &'static str,
    },
    UnknownProperty {
        name: &'static str,
    },
    UnsupportedValueForProperty {
        property: &'static str,
    },
    UnsupportedValue {
        property: Option<&'static str>,
        reason: &'static str,
    },
}

impl ExpectedErrorKind {
    fn assert_matches(&self, actual: &ErrorKind, label: &str) {
        match (self, actual) {
            (Self::InvalidSyntax, ErrorKind::InvalidSyntax { .. }) => {}
            (Self::InvalidSelector, ErrorKind::InvalidSelector { .. }) => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { property },
                ErrorKind::UnsupportedValue {
                    property: actual_property,
                    ..
                },
            ) if Some(*property) == actual_property.as_deref() => {}
            (
                Self::InvalidSyntaxOrUnsupportedValueForProperty { .. },
                ErrorKind::InvalidSyntax { .. },
            ) => {}
            (Self::UnsupportedAtRule { name }, ErrorKind::UnsupportedAtRule { name: actual })
                if name == actual => {}
            (Self::UnknownProperty { name }, ErrorKind::UnknownProperty { name: actual })
                if name == actual => {}
            (
                Self::UnsupportedValueForProperty { property },
                ErrorKind::UnsupportedValue {
                    property: actual_property,
                    ..
                },
            ) if Some(*property) == actual_property.as_deref() => {}
            (
                Self::UnsupportedValue { property, reason },
                ErrorKind::UnsupportedValue {
                    property: actual_property,
                    reason: actual_reason,
                },
            ) if *property == actual_property.as_deref() && *reason == actual_reason => {}
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
            matches!(error.kind(), ErrorKind::UnknownProperty { .. }),
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
    let [declaration] = rule.declarations() else {
        panic!("{input} should parse exactly one declaration");
    };
    declaration.clone()
}
