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

//! Types and functions for performing lexical analysis of Luna source code.

use std::ops::Range;

use derive_more::Display;
use logos::Logos;

/// A span of bytes in some source code.
pub type Span = Range<usize>;

/// The lexical analyser for Luna source code.
///
/// This struct is, in essence, a representation of some source code as an iterator of
/// [`Token`]s.
pub struct Lexer<'a> {
  /// The wrapped [`logos`] lexer struct.
  inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
  /// Create a new lexer over a given input string.
  pub fn new(input: &'a str) -> Self {
    Self {
      inner: TokenKind::lexer(input),
    }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let lexeme = self.inner.slice();
    let span = self.inner.span();
    let (start, end) = (span.start, span.end);

    Some(Self::Item {
      kind,
      lexeme,
      span: Span { start, end },
    })
  }
}

/// A token produced by a [`Lexer`].
#[derive(Debug, Display, Eq, PartialEq, Clone)]
#[display(fmt = "{}", kind)]
pub struct Token<'a> {
  /// The lexical category of this token.
  // #[display(fmt = "{}", kind)]
  pub kind: TokenKind,
  /// The lexeme that matched the pattern for this token.
  pub lexeme: &'a str,
  /// The span of text in the source code that covers the lexeme.
  pub span: Span,
}

/// The lexical category of a [`Token`].
#[derive(Logos, Debug, Display, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
  /// A left bracket `(` character.
  #[display(fmt = "(")]
  #[token("(")]
  LParen,
  /// A right bracket `)` character.
  #[display(fmt = ")")]
  #[token(")")]
  RParen,
  /// A left square bracket `[` character.
  #[display(fmt = "[")]
  #[token("[")]
  LBracket,
  /// A right square bracket `]` character.
  #[display(fmt = "]")]
  #[token("]")]
  RBracket,
  /// A left brace `{` character.
  #[display(fmt = "{{")]
  #[token("{")]
  LBrace,
  /// A right brace `}` character.
  #[display(fmt = "}}")]
  #[token("}")]
  RBrace,

  // TODO: Check the Scheme standard to see what kinds of symbols are allowed.
  /// A symbol (an interned kind of string).
  #[regex(r"(\p{XID_Continue}|\+|-|\*|/|\?|!)+")]
  Symbol,
  /// A string literal.
  #[regex(r#""([^"\\]|\\.)*""#)]
  String,
  // NOTE: Int has a higher priority in order to avoid ambiguity with Symbol.
  /// An integer literal.
  #[regex(r"(\+|-)?[0-9]+", priority = 2)]
  Int,
  /// A boolean literal.
  #[regex(r"true|false")]
  Bool,

  /// A whitespace character, where 'whitespace' is any character having the
  /// `Pattern_White_Space` Unicode property.
  #[regex(r"\p{Pattern_White_Space}+", logos::skip)]
  Whitespace,

  /// A 'token' used for indicating errors encountered during lexical analysis.
  #[error]
  Error,
}

/// Get the matching token for a given token from a token pair.
pub fn get_matching(token_kind: TokenKind) -> TokenKind {
  use TokenKind::*;

  // TODO: Perhaps devise a cleaner way of handling this. This function is principally
  //       required by `parse_list` in the parser module.
  match token_kind {
    LParen => RParen,
    LBracket => RBracket,
    LBrace => RBrace,
    RParen => LParen,
    RBracket => LBracket,
    RBrace => LBrace,
    _ => panic!("Delimiter token expected"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use TokenKind::*;

  fn check(input: &str, token: TokenKind) {
    let mut lexer = TokenKind::lexer(input);

    assert_eq!(lexer.next(), Some(token));
    assert_eq!(lexer.slice(), input);
  }

  #[test]
  fn lex_parens() {
    check("(", LParen);
    check(")", RParen);
  }

  #[test]
  fn lex_brackets() {
    check("[", LBracket);
    check("]", RBracket);
  }

  #[test]
  fn lex_braces() {
    check("{", LBrace);
    check("}", RBrace);
  }

  #[test]
  fn lex_symbol() {
    check("foo", Symbol);
    check("foo123", Symbol);
    check("λ", Symbol);
    check("+", Symbol);
    check("-", Symbol);
    check("*", Symbol);
    check("/", Symbol);
    check("long-function-name", Symbol);
    check("eq?", Symbol);
    check("set!", Symbol);
  }

  #[test]
  fn lex_string() {
    check("\"foo\"", String);
    check("\"\\\"bar\\\"\"", String);
  }

  #[test]
  fn lex_int() {
    check("0", Int);
    check("5", Int);

    check("31", Int);

    check("+6", Int);
    check("-1", Int);
  }

  #[test]
  fn lex_bool() {
    check("true", Bool);
    check("false", Bool);
  }

  #[test]
  fn ignore_whitespace() {
    let mut lexer = TokenKind::lexer(" ");
    assert_eq!(lexer.next(), None);

    let mut lexer = TokenKind::lexer("    ");
    assert_eq!(lexer.next(), None);

    let mut lexer = TokenKind::lexer("\n");
    assert_eq!(lexer.next(), None);

    let mut lexer = TokenKind::lexer("\t \n");
    assert_eq!(lexer.next(), None);
  }
}
