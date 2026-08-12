use std::cell::{Cell, RefCell};
use std::rc::Rc;

use cssparser::{ParseError, Parser, ParserInput, Token};

use crate::error::{Error, is_nesting_limit_error, nesting_limit};
use crate::source::CssSourcePosition;
use crate::syntax::CssSelector;

pub(super) const STRUCTURAL_NESTING_LIMIT: u32 = 256;

/// Parser-owned algorithm state shared by structural and component-value paths.
///
/// The stylesheet root begins at depth zero. A checked guard is the only way to
/// enter a rule block, component-value block, or function, so C04 can extend the
/// same counter without changing the limit or its boundary meaning.
#[derive(Clone)]
pub(crate) struct RecoveryState {
    depth: Rc<Cell<u32>>,
    style_context_captures: StyleContextCaptures,
}

impl RecoveryState {
    pub(super) fn at_depth(depth: u32, style_context_captures: StyleContextCaptures) -> Self {
        Self {
            depth: Rc::new(Cell::new(depth)),
            style_context_captures,
        }
    }

    pub(super) fn record_style_context(
        &self,
        content_start: usize,
        selectors: &[CssSelector],
        position: CssSourcePosition,
    ) -> bool {
        self.style_context_captures
            .record(content_start, selectors, position)
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
        source: &'i str,
        input: &Parser<'i, '_>,
        enclosing_production: &'static str,
    ) -> Result<(), ParseError<'i, Error>> {
        scan_nested_tokens(
            source,
            input.position().byte_index(),
            self.depth.get(),
            enclosing_production,
            ScanBoundary::DeclarationValue,
        )
    }

    pub(super) fn check_failed_rule_block<'i>(
        &self,
        source: &'i str,
        input: &Parser<'i, '_>,
        enclosing_production: &'static str,
    ) -> Option<ParseError<'i, Error>> {
        let content_start = input.position().byte_index();
        let opening_offset = content_start.saturating_sub(1);
        if self.depth.get() >= STRUCTURAL_NESTING_LIMIT {
            return Some(nesting_limit(
                source,
                opening_offset,
                STRUCTURAL_NESTING_LIMIT,
                enclosing_production,
            ));
        }
        scan_nested_tokens(
            source,
            content_start,
            self.depth.get() + 1,
            enclosing_production,
            ScanBoundary::FailedCurlyBlock,
        )
        .err()
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

#[derive(Clone, Default)]
pub(super) struct StyleContextCaptures {
    entries: Rc<RefCell<Vec<StyleContextCapture>>>,
}

struct StyleContextCapture {
    content_start: usize,
    selectors: Option<Vec<CssSelector>>,
    position: Option<CssSourcePosition>,
}

impl StyleContextCaptures {
    pub(super) fn register(&self, content_start: usize) {
        let mut entries = self.entries.borrow_mut();
        if entries
            .iter()
            .all(|entry| entry.content_start != content_start)
        {
            entries.push(StyleContextCapture {
                content_start,
                selectors: None,
                position: None,
            });
        }
    }

    fn record(
        &self,
        content_start: usize,
        selectors: &[CssSelector],
        position: CssSourcePosition,
    ) -> bool {
        let mut entries = self.entries.borrow_mut();
        let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.content_start == content_start)
        else {
            return false;
        };
        if entry.selectors.is_none() {
            entry.selectors = Some(selectors.to_vec());
            entry.position = Some(position);
        }
        true
    }

    pub(super) fn context(
        &self,
        content_start: usize,
    ) -> Option<(Vec<CssSelector>, CssSourcePosition)> {
        self.entries
            .borrow()
            .iter()
            .find(|entry| entry.content_start == content_start)
            .and_then(|entry| Some((entry.selectors.clone()?, entry.position?)))
    }
}

const STRUCTURAL_PARSE_CHUNK: usize = 64;

pub(super) struct StructuralPreflight {
    pub(super) unit_start: usize,
    pub(super) unit_end: usize,
    pub(super) parents: Vec<StructuralParent>,
    pub(super) style_context_starts: Vec<usize>,
    pub(super) parent_depth: u32,
    pub(super) outcome: StructuralPreflightOutcome,
}

pub(super) enum StructuralPreflightOutcome {
    Split,
    NestingLimit {
        opening_offset: usize,
        enclosing_production: &'static str,
    },
}

#[derive(Clone, Copy)]
pub(super) struct StructuralParent {
    pub(super) start: usize,
    pub(super) kind: GroupKind,
}

#[derive(Clone, Copy)]
pub(super) enum GroupKind {
    Layer,
    Media,
    Container,
    Scope,
    Style,
    Component,
    Other,
}

impl GroupKind {
    fn production(self) -> &'static str {
        match self {
            Self::Layer => "baseline.rule.layer-block",
            Self::Media => "baseline.rule.media",
            Self::Container => "baseline.rule.container",
            Self::Scope => "baseline.rule.scope",
            Self::Style => "baseline.rule.style",
            Self::Component => "css.declaration",
            Self::Other => "css.qualified-rule",
        }
    }

    fn can_split(self) -> bool {
        !matches!(self, Self::Other)
    }
}

struct StructuralFrame {
    block: BlockKind,
    group: Option<(usize, GroupKind)>,
}

#[derive(Clone)]
struct StructuralGroup {
    start: usize,
    kind: GroupKind,
    style_context_starts: Vec<usize>,
}

pub(super) fn preflight_structural_nesting(
    source: &str,
    base_depth: u32,
    root_style_context: bool,
) -> Option<StructuralPreflight> {
    // Restarting cssparser at each verified token boundary exposes opening and
    // closing tokens without calling `parse_nested_block`; comments, strings,
    // URLs, and escapes therefore keep cssparser's token semantics while this
    // walk keeps its own heap-backed block stack.
    let mut offset = 0;
    let mut frames: Vec<StructuralFrame> = Vec::new();
    let mut groups: Vec<StructuralGroup> = Vec::new();
    let mut unit_starts = vec![None];
    let mut target: Option<StructuralPreflight> = None;
    let mut target_group_depth = 0;

    while let Some((token_start, token_end, token)) = next_source_token(source, offset) {
        offset = token_end;
        if let Some(closing) = closing_block(&token) {
            if let Some(frame) = frames.pop_if(|frame| frame.block == closing)
                && frame.group.is_some()
            {
                groups.pop();
                unit_starts.pop();
                if let Some(parent_start) = unit_starts.last_mut() {
                    *parent_start = None;
                }
                if target.is_some() && groups.len() < target_group_depth {
                    let mut completed = target.take().expect("target exists");
                    completed.unit_end = token_end;
                    return Some(completed);
                }
            }
            continue;
        }

        let directly_in_group = frames.last().is_none_or(|frame| frame.group.is_some());
        if directly_in_group && is_ignored_unit_prefix(&token) {
            continue;
        }
        if directly_in_group && matches!(token, Token::Semicolon) {
            if let Some(unit_start) = unit_starts.last_mut() {
                *unit_start = None;
            }
            continue;
        }
        if directly_in_group
            && unit_starts
                .last()
                .is_some_and(|unit_start| unit_start.is_none())
            && let Some(unit_start) = unit_starts.last_mut()
        {
            *unit_start = Some(token_start);
        }

        let Some(opening) = opening_block(&token) else {
            continue;
        };
        if opening != BlockKind::Curly || !directly_in_group {
            frames.push(StructuralFrame {
                block: opening,
                group: None,
            });
            continue;
        }

        let unit_start = unit_starts
            .last()
            .and_then(|unit_start| *unit_start)
            .unwrap_or(token_start);
        let group = group_kind(source, unit_start, token_start);
        if matches!(group, GroupKind::Component) {
            frames.push(StructuralFrame {
                block: opening,
                group: None,
            });
            continue;
        }
        let global_depth = base_depth.saturating_add(groups.len() as u32 + 1);
        let split_chain =
            group.can_split() && groups.iter().all(|ancestor| ancestor.kind.can_split());
        let style_context_starts = groups
            .last()
            .map(|parent| parent.style_context_starts.clone())
            .unwrap_or_else(|| root_style_context.then_some(0).into_iter().collect());
        if target.is_none() && split_chain {
            let outcome = if global_depth > STRUCTURAL_NESTING_LIMIT {
                Some(StructuralPreflightOutcome::NestingLimit {
                    opening_offset: token_start,
                    enclosing_production: group.production(),
                })
            } else if groups.len() + 1 == STRUCTURAL_PARSE_CHUNK {
                Some(StructuralPreflightOutcome::Split)
            } else {
                None
            };
            if let Some(outcome) = outcome {
                target = Some(StructuralPreflight {
                    unit_start,
                    unit_end: source.len(),
                    parents: groups
                        .iter()
                        .filter(|parent| !matches!(parent.kind, GroupKind::Style))
                        .map(|parent| StructuralParent {
                            start: parent.start,
                            kind: parent.kind,
                        })
                        .collect(),
                    style_context_starts: style_context_starts.clone(),
                    parent_depth: global_depth.saturating_sub(1),
                    outcome,
                });
                target_group_depth = groups.len() + 1;
            }
        }

        let child_style_context_starts = match group {
            GroupKind::Style => {
                let mut starts = style_context_starts;
                starts.push(token_end);
                starts
            }
            GroupKind::Layer | GroupKind::Media | GroupKind::Container
                if !style_context_starts.is_empty() =>
            {
                let mut starts = style_context_starts;
                starts.push(token_end);
                starts
            }
            GroupKind::Layer
            | GroupKind::Media
            | GroupKind::Container
            | GroupKind::Scope
            | GroupKind::Component
            | GroupKind::Other => Vec::new(),
        };
        groups.push(StructuralGroup {
            start: unit_start,
            kind: group,
            style_context_starts: child_style_context_starts,
        });
        unit_starts.push(None);
        frames.push(StructuralFrame {
            block: opening,
            group: Some((unit_start, group)),
        });
    }

    target
}

fn is_ignored_unit_prefix(token: &Token<'_>) -> bool {
    matches!(
        token,
        Token::WhiteSpace(_) | Token::Comment(_) | Token::CDO | Token::CDC
    )
}

fn group_kind(source: &str, unit_start: usize, opening_offset: usize) -> GroupKind {
    let Some(prelude) = source.get(unit_start..opening_offset) else {
        return GroupKind::Other;
    };
    let trimmed = prelude.trim_start();
    let Some(after_at) = trimmed.strip_prefix('@') else {
        return if looks_like_custom_declaration(trimmed) {
            GroupKind::Component
        } else {
            GroupKind::Style
        };
    };
    let name_end = after_at
        .find(|character: char| !character.is_alphanumeric() && character != '-')
        .unwrap_or(after_at.len());
    match after_at
        .get(..name_end)
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "layer" => GroupKind::Layer,
        "media" => GroupKind::Media,
        "container" => GroupKind::Container,
        "scope" => GroupKind::Scope,
        _ => GroupKind::Other,
    }
}

fn looks_like_custom_declaration(prelude: &str) -> bool {
    let mut input = ParserInput::new(prelude);
    let mut parser = Parser::new(&mut input);
    parser
        .expect_ident_cloned()
        .ok()
        .is_some_and(|name| name.starts_with("--") && parser.expect_colon().is_ok())
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum BlockKind {
    Parenthesis,
    Square,
    Curly,
}

#[derive(Clone, Copy)]
enum ScanBoundary {
    DeclarationValue,
    FailedCurlyBlock,
}

fn scan_nested_tokens<'i>(
    source: &'i str,
    start: usize,
    base_depth: u32,
    enclosing_production: &'static str,
    boundary: ScanBoundary,
) -> Result<(), ParseError<'i, Error>> {
    let mut offset = start.min(source.len());
    let mut blocks = Vec::new();
    while let Some((token_start, token_end, token)) = next_source_token(source, offset) {
        offset = token_end;
        if let Some(closing) = closing_block(&token) {
            if blocks.last() == Some(&closing) {
                blocks.pop();
                continue;
            }
            if blocks.is_empty()
                && matches!(boundary, ScanBoundary::FailedCurlyBlock)
                && closing == BlockKind::Curly
            {
                return Ok(());
            }
            if blocks.is_empty()
                && matches!(boundary, ScanBoundary::DeclarationValue)
                && closing == BlockKind::Curly
            {
                return Ok(());
            }
            continue;
        }
        if blocks.is_empty()
            && matches!(boundary, ScanBoundary::DeclarationValue)
            && matches!(token, Token::Semicolon)
        {
            return Ok(());
        }
        if let Some(opening) = opening_block(&token) {
            let nested_depth = base_depth.saturating_add(blocks.len() as u32);
            if nested_depth >= STRUCTURAL_NESTING_LIMIT {
                return Err(nesting_limit(
                    source,
                    token_start,
                    STRUCTURAL_NESTING_LIMIT,
                    enclosing_production,
                ));
            }
            blocks.push(opening);
        }
    }
    Ok(())
}

fn next_source_token<'i>(source: &'i str, offset: usize) -> Option<(usize, usize, Token<'i>)> {
    let remaining = source.get(offset..)?;
    if remaining.is_empty() {
        return None;
    }
    let mut input = ParserInput::new(remaining);
    let mut parser = Parser::new(&mut input);
    let token = parser
        .next_including_whitespace_and_comments()
        .ok()?
        .clone();
    let token_end = offset.saturating_add(parser.position().byte_index());
    (token_end > offset).then_some((offset, token_end, token))
}

fn opening_block(token: &Token<'_>) -> Option<BlockKind> {
    match token {
        Token::Function(_) | Token::ParenthesisBlock => Some(BlockKind::Parenthesis),
        Token::SquareBracketBlock => Some(BlockKind::Square),
        Token::CurlyBracketBlock => Some(BlockKind::Curly),
        _ => None,
    }
}

fn closing_block(token: &Token<'_>) -> Option<BlockKind> {
    match token {
        Token::CloseParenthesis => Some(BlockKind::Parenthesis),
        Token::CloseSquareBracket => Some(BlockKind::Square),
        Token::CloseCurlyBracket => Some(BlockKind::Curly),
        _ => None,
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

pub(super) fn first_non_trivia_position(
    source: &str,
    member_start: usize,
    member_end: usize,
) -> CssSourcePosition {
    let bounded_start = member_start.min(source.len());
    let bounded_end = member_end.min(source.len()).max(bounded_start);
    let Some(member) = source.get(bounded_start..bounded_end) else {
        return CssSourcePosition::from_byte_offset_in(source, bounded_end);
    };
    let mut input = ParserInput::new(member);
    let mut parser = Parser::new(&mut input);
    loop {
        let token_start = parser.position().byte_index();
        match parser.next_including_whitespace_and_comments() {
            Ok(Token::WhiteSpace(_) | Token::Comment(_)) => {}
            Ok(_) => {
                return CssSourcePosition::from_byte_offset_in(
                    source,
                    bounded_start.saturating_add(token_start),
                );
            }
            Err(_) => return CssSourcePosition::from_byte_offset_in(source, bounded_end),
        }
    }
}

pub(super) fn comma_member_span(
    source: &str,
    member_start: usize,
    member_end: usize,
    following_comma: Option<(usize, usize)>,
    preceding_comma: Option<(usize, usize)>,
) -> Option<crate::CssSourceSpan> {
    let (span_start, span_end) = if member_start < member_end {
        (member_start, member_end)
    } else if let Some(comma) = following_comma {
        comma
    } else if let Some(comma) = preceding_comma {
        comma
    } else {
        (member_start, member_end)
    };
    crate::CssSourceSpan::new(
        CssSourcePosition::from_byte_offset_in(source, span_start),
        CssSourcePosition::from_byte_offset_in(source, span_end),
    )
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
