// SPDX-FileCopyrightText: 2022 Severen Redwood <me@severen.dev>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Lexical analyser for Luna source code.

use derive_more::Display;
use logos::Logos;

use crate::syntax::Span;

/// A token produced by a [`Lexer`].
#[derive(Copy, Clone, Eq, PartialEq, Display, Debug)]
#[display(fmt = "{kind}")]
pub struct Token<'a> {
  /// The lexical category of this token.
  pub kind: TokenKind,
  /// The lexeme that matched the pattern for this token.
  pub lexeme: &'a str,
  /// The span of text in the source code that covers the lexeme.
  pub span: Span,
}

/// The lexical category of a [`Token`].
#[derive(Logos, Copy, Clone, Eq, PartialEq, Display, Debug)]
pub enum TokenKind {
  /// A left bracket `(` character.
  #[display(fmt = "`(`")]
  #[token("(")]
  LParen,
  /// A right bracket `)` character.
  #[display(fmt = "`)`")]
  #[token(")")]
  RParen,
  /// A left square bracket `[` character.
  #[display(fmt = "`[`")]
  #[token("[")]
  LBracket,
  /// A right square bracket `]` character.
  #[display(fmt = "`]`")]
  #[token("]")]
  RBracket,
  /// A left brace `{` character.
  #[display(fmt = "`{{`")]
  #[token("{")]
  LBrace,
  /// A right brace `}` character.
  #[display(fmt = "`}}`")]
  #[token("}")]
  RBrace,

  // The set of extended identifier characters conforms to the minimum set required by
  // the R7RS (Small) specification.
  /// A symbol (an interned kind of string).
  #[display(fmt = "symbol")]
  #[regex(r"(\p{XID_Continue}|!|\$|%|\*|\+|-|\.|/|:|<|=|>|\?|@|\^|_|~)+")]
  Symbol,
  /// A string literal.
  #[display(fmt = "string literal")]
  #[regex(r#""([^"\\]|\\.)*""#)]
  String,
  // NOTE: Int has a higher priority in order to avoid ambiguity with Symbol.
  /// An integer literal.
  #[display(fmt = "integer literal")]
  #[regex(r"(\+|-)?[0-9]+", priority = 2)]
  Int,
  /// A Boolean literal.
  #[display(fmt = "Boolean literal")]
  #[regex(r"#t|#f|#true|#false")]
  Bool,

  /// A 'token' used for indicating errors encountered during lexical analysis.
  #[regex(r"\p{Pattern_White_Space}+", logos::skip)] // Throw away whitespace...
  #[regex(r"|;[^\r\n]*(\r\n|\n)?", logos::skip)] // ...and line comments.
  #[error]
  Invalid,
}

impl TokenKind {
  /// Get the opening token for this token if it has one.
  pub fn opener(&self) -> TokenKind {
    use TokenKind::*;

    match self {
      RParen => LParen,
      RBracket => LBracket,
      RBrace => LBrace,
      _ => panic!("expected a closing delimiter token"),
    }
  }

  /// Get the closing token for this token if it has one.
  pub fn closer(&self) -> TokenKind {
    use TokenKind::*;

    match self {
      LParen => RParen,
      LBracket => RBracket,
      LBrace => RBrace,
      _ => panic!("expected an opening delimiter token"),
    }
  }
}

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
    Self { inner: TokenKind::lexer(input) }
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let lexeme = self.inner.slice();
    let span = self.inner.span();
    // Convert from an std::ops::Range to a crate::syntax::Span.
    let span = Span { start: span.start, end: span.end };

    Some(Self::Item { kind, lexeme, span })
  }
}

#[cfg(test)]
mod tests {
  use TokenKind::*;

  use super::*;

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
    check("#t", Bool);
    check("#f", Bool);
    check("#true", Bool);
    check("#false", Bool);
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

  #[test]
  fn ignore_comments() {
    let mut lexer = TokenKind::lexer("; Hi!");
    assert_eq!(lexer.next(), None);

    let mut lexer = TokenKind::lexer("; Hi!\n");
    assert_eq!(lexer.next(), None);

    let mut lexer = TokenKind::lexer("; Hi!\r\n");
    assert_eq!(lexer.next(), None);
  }
}
