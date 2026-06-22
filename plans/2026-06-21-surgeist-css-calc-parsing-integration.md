# Surgeist CSS Calc Parsing Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Parse strict CSS `calc()` length syntax into `surgeist-style` authored calc values without taking ownership of style validation or layout lowering.

**Architecture:** `surgeist-css` remains a strict syntax-ingestion crate. It recognizes CSS `calc()` where current length syntax is accepted, constructs `surgeist_style::CalcLength`, and leaves property validation, fingerprints, percentage normalization, and layout lowering to `surgeist-style`.

**Tech Stack:** Rust 2024 in `surgeist-css`, `cssparser = 0.37.0`, `surgeist-style` as the typed value dependency, focused tests with `cargo test -p surgeist-css`, formatting with `cargo fmt --check`, linting with `cargo clippy -p surgeist-css --all-targets -- -D warnings`.

---

## Non-Negotiable Constraints

- Work only in the `surgeist-css` repo. Do not edit `../surgeist-style`, `../surgeist-layout`, or the top-level `../surgeist` repo from this plan.
- This crate parses syntax only. Do not add layout calc handles, layout stores, resolver hooks, or basis-dependent behavior here.
- Preserve strict parsing: unsupported units, unsupported functions, invalid nesting, extra tokens, and malformed calc expressions are errors.
- Preserve authored CSS percentage spelling: parsing `10%` produces `surgeist_style::CalcLength::percent(10.0)`, not `0.10`.
- Do not add lint suppressions. Fix warnings by improving code shape.
- Commit after each task with the message listed in that task.

## Required Style Contract

This plan depends on `surgeist-style` exposing:

```rust
pub enum CalcLength;
pub struct CalcLengthTerm;

impl CalcLength {
    pub fn px(value: f32) -> Self;
    pub fn percent(value: f32) -> Self;
    pub fn sum(terms: impl IntoIterator<Item = CalcLengthTerm>) -> Self;
    pub fn uses_percentage(&self) -> bool;
    pub fn to_css_string(&self) -> String;
}

impl CalcLengthTerm {
    pub fn add(value: CalcLength) -> Self;
    pub fn sub(value: CalcLength) -> Self;
}

pub enum Length {
    Calc(CalcLength),
}
```

If those APIs differ, stop and open an upstream `surgeist-style` issue or update this plan with the exact final API before implementing parser changes.

## File Map

- Modify: `src/lib.rs`
  - Extend the existing strict length parser to recognize `calc()`.
  - Add parser helpers for calc sums, signed terms, nested calc, px terms, percent terms, and zero terms.
  - Add focused parser tests for supported and rejected calc syntax.

### Task 1: Parse Calc Lengths

**Files:**
- Modify: `src/lib.rs`

- [ ] **Step 1: Confirm style calc API exists**

Run:

```sh
rg "CalcLength|CalcLengthTerm|Length::Calc" ../surgeist-style/src ../surgeist-style/api/public-api.txt
```

Expected: the required style contract symbols are visible. If they are not visible, stop and wait for the style crate plan to land.

- [ ] **Step 2: Write failing parser tests**

Add this test module at the end of `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn declaration_value(input: &str, property: style::Property) -> style::Value {
        let sheet = parse_sheet(input).unwrap();
        sheet.rules()[0]
            .declarations
            .get(property)
            .unwrap()
            .clone()
    }

    #[test]
    fn parses_calc_width_as_style_calc_length() {
        let value = declaration_value(
            ".panel { width: calc(20px + 10%); }",
            style::Property::Width,
        );

        match value {
            style::Value::Length(style::Length::Calc(calc)) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(20px + 10%)");
            }
            other => panic!("expected calc length, got {other:?}"),
        }
    }

    #[test]
    fn parses_nested_calc_width_with_subtraction() {
        let value = declaration_value(
            ".panel { width: calc(100% - calc(12px + 3%)); }",
            style::Property::Width,
        );

        match value {
            style::Value::Length(style::Length::Calc(calc)) => {
                assert!(calc.uses_percentage());
                assert_eq!(calc.to_css_string(), "calc(100% - calc(12px + 3%))");
            }
            other => panic!("expected nested calc length, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unsupported_calc_units() {
        let error = parse_sheet(".panel { width: calc(1em + 2px); }").unwrap_err();
        assert!(error.message().contains("unsupported calc length unit"));
    }

    #[test]
    fn rejects_unknown_calc_functions() {
        let error = parse_sheet(".panel { width: min(10px, 20px); }").unwrap_err();
        assert!(error.message().contains("unsupported length function"));
    }
}
```

- [ ] **Step 3: Run tests to verify failure**

Run:

```sh
cargo test -p surgeist-css tests::parses_calc_width_as_style_calc_length tests::parses_nested_calc_width_with_subtraction tests::rejects_unsupported_calc_units tests::rejects_unknown_calc_functions
```

Expected: parser tests fail because `parse_length` does not recognize function tokens.

- [ ] **Step 4: Import calc style types**

Update the existing `surgeist_style` import list:

```rust
use surgeist_style::{
    self as style, AlignItems, BoxSizing, CalcLength, CalcLengthTerm, Color, Declarations,
    Direction, Display, Edges, FlexDirection, FlexWrap, GridFlowTolerance, LayoutPosition, Length,
    Overflow, OverflowAxes, Property, Selector, Sheet, Value,
};
```

- [ ] **Step 5: Extend `parse_length` for functions**

In `parse_length`, add a `Token::Function` arm before the final unexpected-token arm:

```rust
Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
    let calc = input.parse_nested_block(parse_calc_length)?;
    Ok(Length::Calc(calc))
}
Token::Function(name) => Err(error_at(
    location,
    format!("unsupported length function `{name}`"),
)),
```

- [ ] **Step 6: Add calc parser helpers**

Add these helpers near `parse_length`:

```rust
fn parse_calc_length<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CalcLength, ParseError<'i, Error>> {
    let mut terms = Vec::new();
    terms.push(CalcLengthTerm::add(parse_calc_component(input)?));

    while !input.is_exhausted() {
        let location = input.current_source_location();
        let operator = match input.next().map_err(basic)? {
            Token::Delim('+') => CalcLengthTerm::add,
            Token::Delim('-') => CalcLengthTerm::sub,
            token => {
                return Err(error_at(
                    location,
                    format!("expected calc operator, got `{}`", token.to_css_string()),
                ));
            }
        };
        let component = parse_calc_component(input)?;
        terms.push(operator(component));
    }

    Ok(CalcLength::sum(terms))
}

fn parse_calc_component<'i, 't>(
    input: &mut Parser<'i, 't>,
) -> std::result::Result<CalcLength, ParseError<'i, Error>> {
    let location = input.current_source_location();
    match input.next().map_err(basic)? {
        Token::Dimension { value, unit, .. } if unit.eq_ignore_ascii_case("px") => {
            Ok(CalcLength::px(*value))
        }
        Token::Dimension { unit, .. } => Err(error_at(
            location,
            format!("unsupported calc length unit `{unit}`"),
        )),
        Token::Percentage { unit_value, .. } => Ok(CalcLength::percent(*unit_value * 100.0)),
        Token::Number { value, .. } if *value == 0.0 => Ok(CalcLength::px(0.0)),
        Token::Function(name) if name.eq_ignore_ascii_case("calc") => {
            input.parse_nested_block(parse_calc_length)
        }
        Token::Function(name) => Err(error_at(
            location,
            format!("unsupported calc function `{name}`"),
        )),
        token => Err(error_at(
            location,
            format!("unexpected calc token `{}`", token.to_css_string()),
        )),
    }
}
```

- [ ] **Step 7: Run focused parser tests**

Run:

```sh
cargo test -p surgeist-css tests::parses_calc_width_as_style_calc_length tests::parses_nested_calc_width_with_subtraction tests::rejects_unsupported_calc_units tests::rejects_unknown_calc_functions
```

Expected: all focused parser tests pass.

- [ ] **Step 8: Commit**

```sh
git add -- src/lib.rs
git commit -m "css: parse calc length syntax"
```

### Task 2: Calc Coverage For Existing Length Consumers

**Files:**
- Modify: `src/lib.rs`

- [ ] **Step 1: Write failing shorthand and gap tests**

Add these tests to the `tests` module:

```rust
#[test]
fn parses_calc_in_edge_shorthands() {
    let sheet = parse_sheet(".panel { margin: calc(4px + 1%) 2px; }").unwrap();
    let edges = match sheet.rules()[0]
        .declarations
        .get(style::Property::Margin)
        .unwrap()
    {
        style::Value::Edges(edges) => edges,
        other => panic!("expected edges, got {other:?}"),
    };

    assert!(matches!(edges.top, style::Length::Calc(_)));
    assert_eq!(edges.right, style::Length::px(2.0));
    assert!(matches!(edges.bottom, style::Length::Calc(_)));
    assert_eq!(edges.left, style::Length::px(2.0));
}

#[test]
fn parses_normal_gap_without_treating_it_as_calc() {
    let value = declaration_value(".panel { gap: normal; }", style::Property::RowGap);
    assert_eq!(value, style::Value::Length(style::Length::NORMAL));
}

#[test]
fn parses_calc_gap() {
    let value = declaration_value(
        ".panel { gap: calc(8px + 2%); }",
        style::Property::RowGap,
    );
    assert!(matches!(value, style::Value::Length(style::Length::Calc(_))));
}
```

- [ ] **Step 2: Run tests to verify current behavior**

Run:

```sh
cargo test -p surgeist-css tests::parses_calc_in_edge_shorthands tests::parses_normal_gap_without_treating_it_as_calc tests::parses_calc_gap
```

Expected: shorthand and calc gap tests pass if Task 1 integrated through `parse_length`; otherwise they expose the remaining parse path that still bypasses `parse_length`.

- [ ] **Step 3: Fix any bypassing parse path**

If the tests fail, update the failing parser path to call `parse_length(input)` or `parse_gap_length(input)` consistently. For example, `parse_edges` must keep this shape:

```rust
while !input.is_exhausted() {
    values.push(parse_length(input)?);
    if values.len() == 4 && !input.is_exhausted() {
        return Err(custom_error(input, "edge shorthand has too many values"));
    }
}
```

- [ ] **Step 4: Run package checks**

Run:

```sh
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
cargo fmt --check
```

Expected: all commands pass.

- [ ] **Step 5: Commit**

```sh
git add -- src/lib.rs
git commit -m "css: cover calc length consumers"
```

### Task 3: Public API Artifact And Final Verification

**Files:**
- Modify: `api/public-api.txt`

- [ ] **Step 1: Refresh public API artifact**

Run the crate's existing API generator command from its README or current crate convention. If no command is documented, run:

```sh
cargo run --manifest-path api/generator/Cargo.toml > api/public-api.txt
```

Expected: `api/public-api.txt` changes only if the parser crate's public API changed.

- [ ] **Step 2: Final verification**

Run:

```sh
cargo fmt --check
cargo test -p surgeist-css
cargo clippy -p surgeist-css --all-targets -- -D warnings
git diff --check
```

Expected: all commands pass.

- [ ] **Step 3: Commit if the API artifact changed**

If `api/public-api.txt` changed:

```sh
git add -- api/public-api.txt
git commit -m "css: refresh public api after calc parsing"
```

If `api/public-api.txt` did not change, do not create an empty commit.
