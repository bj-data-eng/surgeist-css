use std::cell::Cell;
use std::rc::Rc;

use cssparser::{ParseError, Parser, Token};

use crate::error::{Error, is_nesting_limit_error, nesting_limit};

pub(super) const STRUCTURAL_NESTING_LIMIT: u32 = 256;

/// Parser-owned algorithm state shared by structural and component-value paths.
///
/// The stylesheet root begins at depth zero. A checked guard is the only way to
/// enter a rule block, component-value block, or function, so C04 can extend the
/// same counter without changing the limit or its boundary meaning.
#[derive(Clone)]
pub(crate) struct RecoveryState {
    depth: Rc<Cell<u32>>,
}

impl RecoveryState {
    pub(super) fn new() -> Self {
        Self {
            depth: Rc::new(Cell::new(0)),
        }
    }

    pub(super) fn enter_rule_block<'i>(
        &self,
        source: &str,
        input: &Parser<'i, '_>,
        enclosing_production: &'static str,
    ) -> Result<RecoveryDepthGuard, ParseError<'i, Error>> {
        let opening_offset = input.position().byte_index().saturating_sub(1);
        self.enter(source, opening_offset, enclosing_production)
    }

    pub(super) fn check_component_values<'i>(
        &self,
        source: &str,
        input: &mut Parser<'i, '_>,
        enclosing_production: &'static str,
    ) -> Result<(), ParseError<'i, Error>> {
        let initial = input.state();
        let result = self.scan_component_values(source, input, enclosing_production);
        input.reset(&initial);
        result
    }

    fn scan_component_values<'i>(
        &self,
        source: &str,
        input: &mut Parser<'i, '_>,
        enclosing_production: &'static str,
    ) -> Result<(), ParseError<'i, Error>> {
        loop {
            let token_start = input.position().byte_index();
            let token = match input.next_including_whitespace_and_comments() {
                Ok(token) => token.clone(),
                Err(_) => return Ok(()),
            };
            if matches!(
                token,
                Token::Function(_)
                    | Token::ParenthesisBlock
                    | Token::SquareBracketBlock
                    | Token::CurlyBracketBlock
            ) {
                let _guard = self.enter(source, token_start, enclosing_production)?;
                input.parse_nested_block(|nested| {
                    self.scan_component_values(source, nested, enclosing_production)
                })?;
            }
        }
    }

    fn enter<'i>(
        &self,
        source: &str,
        opening_offset: usize,
        enclosing_production: &'static str,
    ) -> Result<RecoveryDepthGuard, ParseError<'i, Error>> {
        let depth = self.depth.get();
        if depth >= STRUCTURAL_NESTING_LIMIT {
            return Err(nesting_limit(
                source,
                opening_offset,
                STRUCTURAL_NESTING_LIMIT,
                enclosing_production,
            ));
        }
        self.depth.set(depth + 1);
        Ok(RecoveryDepthGuard {
            depth: Rc::clone(&self.depth),
        })
    }
}

pub(super) struct RecoveryDepthGuard {
    depth: Rc<Cell<u32>>,
}

impl Drop for RecoveryDepthGuard {
    fn drop(&mut self) {
        self.depth.set(self.depth.get().saturating_sub(1));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RecoveryLoopOutcome {
    Retained,
    Advanced,
    Terminated,
}

/// One iteration's byte-progress witness for a bounded recovery coordinator.
pub(super) struct RecoveryProgress {
    start_byte: usize,
}

impl RecoveryProgress {
    pub(super) fn record(input: &Parser<'_, '_>) -> Self {
        Self {
            start_byte: input.position().byte_index(),
        }
    }

    pub(super) fn finish(self, input: &mut Parser<'_, '_>, retained: bool) -> RecoveryLoopOutcome {
        if retained {
            return if input.position().byte_index() > self.start_byte {
                RecoveryLoopOutcome::Retained
            } else {
                RecoveryLoopOutcome::Terminated
            };
        }
        if input.position().byte_index() > self.start_byte {
            return RecoveryLoopOutcome::Advanced;
        }
        if input.next_including_whitespace_and_comments().is_ok()
            && input.position().byte_index() > self.start_byte
        {
            RecoveryLoopOutcome::Advanced
        } else {
            RecoveryLoopOutcome::Terminated
        }
    }
}

pub(super) fn recovery_action_for_error(
    error: &ParseError<'_, Error>,
    ordinary: crate::CssRecoveryAction,
) -> crate::CssRecoveryAction {
    if is_nesting_limit_error(error) {
        crate::CssRecoveryAction::StopAtNestingLimit
    } else {
        ordinary
    }
}

#[cfg(test)]
mod tests {
    use cssparser::{Parser, ParserInput};

    use super::{RecoveryLoopOutcome, RecoveryProgress};

    #[test]
    fn structural_recovery_zero_progress_failure_advances_one_token() {
        let mut input = ParserInput::new(";later");
        let mut parser = Parser::new(&mut input);
        let progress = RecoveryProgress::record(&parser);

        assert_eq!(
            progress.finish(&mut parser, false),
            RecoveryLoopOutcome::Advanced
        );
        assert_eq!(parser.position().byte_index(), 1);
    }

    #[test]
    fn structural_recovery_zero_progress_at_bounded_end_terminates() {
        let mut input = ParserInput::new("");
        let mut parser = Parser::new(&mut input);
        let progress = RecoveryProgress::record(&parser);

        assert_eq!(
            progress.finish(&mut parser, false),
            RecoveryLoopOutcome::Terminated
        );
    }
}
