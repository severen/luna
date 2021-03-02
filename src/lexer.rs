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

//! This module contains types and functions for performing lexical analysis of
//! Luna source code.

use std::fmt::{self, Display, Formatter};

use logos::Logos;

/// A span of bytes in some source code.
pub type Span = std::ops::Range<usize>;

/// The lexical analyser for Luna source code.
///
/// This struct is, in essence, a representation of some source code as an
/// iterator of [`Token`]s.
pub struct Lexer<'a> {
  /// The wrapped [`logos`] lexer struct.
  inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
  /// Create a new lexer over a given input string.
  pub fn new(input: &'a str) -> Self {
    Self { inner: TokenKind::lexer(input) }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let lexeme = self.inner.slice();
    let span = self.inner.span();

    Some(Self::Item { kind, lexeme, span })
  }
}

/// A token produced by a [`Lexer`].
#[derive(Debug, Eq, PartialEq)]
pub struct Token<'a> {
  /// The lexical category of this token.
  pub kind: TokenKind,
  /// The lexeme that matched the pattern for this token.
  pub lexeme: &'a str,
  /// The span of text in the source code that covers the lexeme.
  pub span: Span,
}

impl<'a> Display for Token<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Display::fmt(&self.kind, f)
  }
}

/// The lexical category of a [`Token`].
#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
  /// A left bracket `(` character.
  #[token("(")]
  LParen,
  /// A right bracket `)` character.
  #[token(")")]
  RParen,
  /// A left square bracket `[` character.
  #[token("[")]
  LBracket,
  /// A right square bracket `]` character.
  #[token("]")]
  RBracket,
  /// A left brace `{` character.
  #[token("{")]
  LBrace,
  /// A right brace `}` character.
  #[token("}")]
  RBrace,

  // TODO: Expand this to include special characters.
  /// An identifier.
  #[regex(r"\p{XID_Continue}+")]
  Ident,
  // We give Int a higher priority to avoid ambiguity with Ident.
  /// An integer literal.
  #[regex(r"(\+|-)?[0-9]+", priority = 2)]
  Int,

  /// A whitespace character, where 'whitespace' is defined to be any character
  /// in the Unicode lexical class `Pattern_White_Space`.
  #[regex(r"\p{Pattern_White_Space}+", logos::skip)]
  Whitespace,

  /// A 'token' used for indicating errors encountered during lexical analysis.
  #[error]
  Error,
}

impl Display for TokenKind {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use TokenKind::*;

    match self {
      LParen => write!(f, "("),
      RParen => write!(f, ")"),
      LBracket => write!(f, "["),
      RBracket => write!(f, "]"),
      LBrace => write!(f, "{{"),
      RBrace => write!(f, "}}"),
      Ident => write!(f, "Ident"),
      Int => write!(f, "Int"),
      Whitespace => write!(f, "Whitespace"),
      Error => write!(f, "Error"),
    }
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
  fn lex_ident() {
    check("foo", Ident);
    check("foo123", Ident);
    check("λ", Ident);
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
