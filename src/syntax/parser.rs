// This file is part of Luna.
//
// Luna is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Luna is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Luna.  If not, see <https://www.gnu.org/licenses/>.

//! Parser for Luna source code.

use std::iter::Peekable;

use crate::syntax::{self, Lexer, Span, TokenKind};

/// A symbolic expression.
#[derive(Eq, PartialEq, Debug)]
pub enum SExpr {
  /// A symbol atom.
  Symbol(String),
  /// A string atom.
  String(String),
  /// An integer atom.
  Int(i32),
  // A Boolean atom.
  Bool(bool),
  /// A list of symbolic expressions.
  List(Vec<SExpr>),
}

/// A specialiation of [`Result`](std::result::Result) for brevity when writing return
/// types for parser functions.
type Result<T> = std::result::Result<T, syntax::Error>;

/// Produce a [`struct@syntax::Error`] and return from the surrounding function.
macro_rules! error {
  ($span: expr, $kind: ident $(,)?) => {
    return Err(syntax::Error { span: $span, kind: syntax::ErrorKind::$kind })
  };
}

/// Parse source code into an abstract syntax tree.
pub fn parse(input: &str) -> Result<Vec<SExpr>> {
  let mut lexer = Lexer::new(strip_shebang(input)).peekable();

  let mut program = Vec::new();
  while let Some(token) = lexer.peek() {
    use TokenKind::*;

    let sexpr = match token.kind {
      Symbol => parse_symbol(&mut lexer),
      String => parse_string(&mut lexer),
      Int => parse_int(&mut lexer),
      Bool => parse_bool(&mut lexer),
      LParen | LBracket | LBrace => parse_list(&mut lexer)?,
      RParen | RBracket | RBrace => {
        error!(Span { start: token.span.start, end: token.span.end }, UnexpectedBracket)
      },
      Invalid => {
        error!(Span { start: token.span.start, end: token.span.end }, InvalidToken)
      },
    };
    program.push(sexpr);
  }

  Ok(program)
}

/// Parse a symbol.
fn parse_symbol(lexer: &mut Peekable<Lexer>) -> SExpr {
  SExpr::Symbol(lexer.next().unwrap().lexeme.to_string())
}

/// Parse a string.
fn parse_string(lexer: &mut Peekable<Lexer>) -> SExpr {
  SExpr::String(lexer.next().unwrap().lexeme.to_string())
}

/// Parse an integer.
fn parse_int(lexer: &mut Peekable<Lexer>) -> SExpr {
  SExpr::Int(lexer.next().unwrap().lexeme.parse().unwrap())
}

/// Parse a boolean.
fn parse_bool(lexer: &mut Peekable<Lexer>) -> SExpr {
  let lexeme = lexer.next().unwrap().lexeme;
  let value = match lexeme {
    "true" => true,
    "false" => false,
    _ => unreachable!(),
  };

  SExpr::Bool(value)
}

/// Parse a list.
fn parse_list(lexer: &mut Peekable<Lexer>) -> Result<SExpr> {
  let mut list = Vec::new();

  // NOTE: It is an invariant that an opening bracket be present, so we can consume
  //       it and unwrap.
  let opener = lexer.next().expect("an opening bracket should always be present");
  let Span { start: list_start, end: mut list_end } = opener.span;

  while let Some(token) = lexer.peek() {
    use TokenKind::*;

    list_end = token.span.end;
    list.push(match token.kind {
      Symbol => parse_symbol(lexer),
      String => parse_string(lexer),
      Int => parse_int(lexer),
      Bool => parse_bool(lexer),
      LParen | LBracket | LBrace => parse_list(lexer)?,
      RParen | RBracket | RBrace => {
        if token.kind != opener.kind.closer() {
          error!(Span { start: list_start, end: list_end }, UnexpectedBracket);
        }
        break;
      },
      Invalid => {
        error!(Span { start: token.span.start, end: token.span.end }, InvalidToken)
      },
    })
  }

  // Consume the closing bracket.
  if lexer.next().is_none() {
    error!(Span { start: list_start, end: list_end }, UnmatchedBracket);
  }

  Ok(SExpr::List(list))
}

// TODO: Move this into a module containing program file abstractions.
/// Strip the shebang line from a string if one is present.
pub fn strip_shebang(input: &str) -> &str {
  if input.starts_with("#!") {
    // The byte index of the first character after the shebang line.
    let i = input.find('\n').map(|i| i + 1).unwrap_or_else(|| input.len());
    &input[i..]
  } else {
    input
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_program() -> Result<()> {
    parse("(defn fac [n]\n(fac (minus n 1)))\n\n(print (fac 5))")?;
    Ok(())
  }

  #[test]
  fn parse_symbol() -> Result<()> {
    parse("hello")?;
    parse("foo bar")?;
    parse("foo\nbar")?;

    Ok(())
  }

  #[test]
  fn parse_string() -> Result<()> {
    parse("\"foo\"")?;
    parse("\"\\\"bar\\\"\"")?;

    Ok(())
  }

  #[test]
  fn parse_int() -> Result<()> {
    parse("10")?;
    parse("0 11")?;
    parse("0 -11")?;

    Ok(())
  }

  #[test]
  fn parse_bool() -> Result<()> {
    parse("true")?;
    parse("false")?;

    Ok(())
  }

  #[test]
  fn parse_list() -> Result<()> {
    // Can we parse empty lists?
    parse("()")?;
    parse("[]")?;
    parse("{}")?;

    // Can we parse normal lists?
    parse("(1 2 3)")?;
    parse("[1 2 3]")?;
    parse("{1 2 3}")?;

    // Can we parse nested lists?
    parse("(1 [2 {3}])")?;
    parse("{1 [2 3]}")?;

    Ok(())
  }

  #[test]
  fn ignore_shebang() {
    const PROGRAM1: &str = "#!/usr/bin/env luna\n(define x 10)\n";
    assert_eq!(strip_shebang(PROGRAM1), "(define x 10)\n");

    const PROGRAM2: &str = "#!/usr/bin/env luna";
    assert!(strip_shebang(PROGRAM2).is_empty());

    const PROGRAM3: &str = "#!/usr/bin/env luna\n";
    assert!(strip_shebang(PROGRAM3).is_empty());
  }
}
