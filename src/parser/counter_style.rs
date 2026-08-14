use cssparser::{
    AtRuleParser, CowRcStr, DeclarationParser, ParseError, Parser, ParserState,
    QualifiedRuleParser, RuleBodyItemParser, RuleBodyParser, match_ignore_ascii_case,
};

use super::background::parse_url;
use super::recovery::{RecoveryLoopOutcome, RecoveryProgress, RecoveryState};
use super::values::parse_integer;
use super::{block_item_diagnostic, is_declaration_recovery_unit, parse_descriptor_boundary};
use crate::error::{
    Error, basic, descriptor_name_error, invalid_descriptor_combination, unsupported_value,
    unsupported_value_at, with_descriptor_context,
};
use crate::syntax::*;

pub(super) fn parse_counter_style_name<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssCounterStyleName, ParseError<'i, Error>> {
    let location = input.current_source_location();
    let name = input.expect_ident_cloned().map_err(basic)?;
    let name = CssCounterStyleName::try_new(name.to_string()).ok_or_else(|| {
        unsupported_value_at(
            location,
            None,
            "counter-style names exclude CSS-wide keywords and `none`",
        )
    })?;
    input.expect_exhausted().map_err(basic)?;
    Ok(name)
}

pub(super) fn parse_counter_style_rule<'i, 't>(
    source: &'i str,
    name: CssCounterStyleName,
    input: &mut Parser<'i, 't>,
    start: &ParserState,
    diagnostics: &mut Vec<crate::CssRecoveryDiagnostic>,
    recovery: RecoveryState,
) -> Result<CssCounterStyleRule, ParseError<'i, Error>> {
    let mut occurrences = Vec::new();
    let mut descriptor_parser = CounterStyleDescriptorParser { source, recovery };
    let mut items = RuleBodyParser::new(input, &mut descriptor_parser);

    loop {
        let progress = RecoveryProgress::record(items.input);
        let Some(item) = items.next() else {
            break;
        };
        let retained = item.is_ok();
        let progress_outcome = progress.finish(items.input, retained);
        let unit_end = items.input.position().byte_index();
        match item {
            Ok(descriptor) => occurrences.push(descriptor),
            Err((error, failed_unit)) if is_declaration_recovery_unit(failed_unit) => {
                if let Some(diagnostic) = block_item_diagnostic(
                    source,
                    error,
                    failed_unit,
                    unit_end,
                    crate::CssRecoveryAction::DropDescriptor,
                ) {
                    diagnostics.push(diagnostic);
                }
            }
            Err((error, _)) => return Err(error),
        }
        if progress_outcome == RecoveryLoopOutcome::Terminated {
            break;
        }
    }

    let descriptors =
        CssCounterStyleDescriptors::from_occurrences(occurrences).map_err(|issue| {
            invalid_descriptor_combination(
                input,
                issue.position(),
                "counter-style",
                issue.responsible(),
                issue.conflicting(),
            )
        })?;

    Ok(CssCounterStyleRule::new(
        name,
        descriptors,
        crate::CssSourcePosition::from_cssparser(start.position(), start.source_location()),
    ))
}

struct CounterStyleDescriptorParser<'s> {
    source: &'s str,
    recovery: RecoveryState,
}

impl<'i> AtRuleParser<'i> for CounterStyleDescriptorParser<'i> {
    type Prelude = ();
    type AtRule = CssCounterStyleDescriptor;
    type Error = Error;
}

impl<'i> QualifiedRuleParser<'i> for CounterStyleDescriptorParser<'i> {
    type Prelude = ();
    type QualifiedRule = CssCounterStyleDescriptor;
    type Error = Error;
}

impl<'i> RuleBodyItemParser<'i, CssCounterStyleDescriptor, Error>
    for CounterStyleDescriptorParser<'i>
{
    fn parse_declarations(&self) -> bool {
        true
    }

    fn parse_qualified(&self) -> bool {
        false
    }
}

impl<'i> DeclarationParser<'i> for CounterStyleDescriptorParser<'i> {
    type Declaration = CssCounterStyleDescriptor;
    type Error = Error;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        declaration_start: &ParserState,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let implicit_closures =
            self.recovery
                .check_component_values(self.source, input, "css.descriptor")?;
        let position = crate::CssSourcePosition::from_cssparser(
            declaration_start.position(),
            declaration_start.source_location(),
        );
        let result = (|| {
            Ok(match_ignore_ascii_case! { &name,
                "system" => CssCounterStyleDescriptor::System(CssDescriptorOccurrence::new(
                    parse_descriptor_boundary(input, "counter-style", "system", parse_system)?,
                    position,
                )),
                "symbols" => CssCounterStyleDescriptor::Symbols(CssDescriptorOccurrence::new(
                    parse_descriptor_boundary(input, "counter-style", "symbols", parse_symbols)?,
                    position,
                )),
                "prefix" => CssCounterStyleDescriptor::Prefix(CssDescriptorOccurrence::new(
                    parse_descriptor_boundary(input, "counter-style", "prefix", parse_symbol)?,
                    position,
                )),
                "suffix" => CssCounterStyleDescriptor::Suffix(CssDescriptorOccurrence::new(
                    parse_descriptor_boundary(input, "counter-style", "suffix", parse_symbol)?,
                    position,
                )),
                _ => return Err(descriptor_name_error(
                    declaration_start.source_location(),
                    "counter-style",
                    name.as_ref(),
                )),
            })
        })()
        .map_err(|error| with_descriptor_context(error, "counter-style", name.as_ref()))?;
        self.recovery.retain_component_closures(implicit_closures);
        Ok(result)
    }
}

fn parse_system<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssCounterStyleSystem, ParseError<'i, Error>> {
    let ident = input.expect_ident_cloned().map_err(basic)?;
    let system = match_ignore_ascii_case! { &ident,
        "cyclic" => CssCounterStyleSystem::Cyclic,
        "numeric" => CssCounterStyleSystem::Numeric,
        "alphabetic" => CssCounterStyleSystem::Alphabetic,
        "symbolic" => CssCounterStyleSystem::Symbolic,
        "additive" => CssCounterStyleSystem::Additive,
        "fixed" => {
            let first_symbol_value = if input.is_exhausted() {
                None
            } else {
                Some(parse_integer(input, "fixed counter-style starting value")?)
            };
            CssCounterStyleSystem::Fixed(CssCounterStyleFixedSystem::new(first_symbol_value))
        },
        "extends" => {
            let location = input.current_source_location();
            let name = input.expect_ident_cloned().map_err(basic)?;
            let name = CssCounterStyleName::try_new(name.to_string()).ok_or_else(|| {
                unsupported_value_at(location, None, "invalid extended counter-style name")
            })?;
            CssCounterStyleSystem::Extends(name)
        },
        _ => return Err(unsupported_value(input, None, "unsupported counter-style system")),
    };
    input.expect_exhausted().map_err(basic)?;
    Ok(system)
}

fn parse_symbols<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssCounterSymbols, ParseError<'i, Error>> {
    let mut symbols = Vec::new();
    while !input.is_exhausted() {
        symbols.push(parse_symbol_component(input)?);
    }
    if symbols.is_empty() {
        Err(unsupported_value(input, None, "symbols must not be empty"))
    } else {
        Ok(CssCounterSymbols::new(symbols))
    }
}

fn parse_symbol<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssCounterSymbol, ParseError<'i, Error>> {
    let symbol = parse_symbol_component(input)?;
    input.expect_exhausted().map_err(basic)?;
    Ok(symbol)
}

fn parse_symbol_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> Result<CssCounterSymbol, ParseError<'i, Error>> {
    if let Ok(value) = input.try_parse(Parser::expect_string_cloned) {
        return CssContentString::try_new(value.to_string())
            .map(CssCounterSymbol::String)
            .ok_or_else(|| unsupported_value(input, None, "counter symbol contains null"));
    }
    if let Ok(value) = input.try_parse(parse_url) {
        return Ok(CssCounterSymbol::Url(value));
    }
    let location = input.current_source_location();
    let ident = input.expect_ident_cloned().map_err(basic)?;
    CssCounterSymbolIdent::try_new(ident.to_string())
        .map(CssCounterSymbol::Ident)
        .ok_or_else(|| unsupported_value_at(location, None, "invalid custom-ident counter symbol"))
}
