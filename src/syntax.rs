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

//! Lexing and parsing of Luna source code.

use thiserror::Error;

mod lexer;
mod parser;

pub use lexer::{Lexer, Token, TokenKind};
pub use parser::{parse, SExpr};

/// A byte position within an input stream.
pub type BytePos = usize;

/// A span of bytes within an input stream.
///
/// Specifically, a `Span` is a range `[a, b)` for integers `a` and `b` such that `a <
/// b`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Span {
  /// The index of the first byte in the range.
  pub start: BytePos,
  /// The index of the byte after the last byte in the range.
  pub end: BytePos,
}

/// A syntax error.
///
/// This error struct is emitted by the [`parse`] function if it encounters a
/// span of source code containing a syntactical error. The specific kind of error is
/// denoted by the [`ErrorKind`] enum.
#[derive(Copy, Clone, Eq, PartialEq, Error, Debug)]
#[error("{kind}")]
pub struct Error {
  /// The span of source code in which this syntax error was encountered.
  pub span: Span,
  /// The kind of this syntax error.
  pub kind: ErrorKind,
}

/// The kind of a syntax error.
#[derive(Copy, Clone, Eq, PartialEq, Error, Debug)]
pub enum ErrorKind {
  /// An invalid token was encountered.
  #[error("Invalid token")]
  InvalidToken,
  /// An unexpected token was encountered.
  #[error("unexpected `{}`", .found)]
  UnexpectedToken {
    /// The unexpected token that was encountered.
    found: TokenKind,
  },
  /// An unexpected kind of closing bracket was encountered.
  #[error("expected `{}` to close preceding `{}`, found `{}` instead", .expected, .expected.opener(), .found)]
  UnexpectedBracket {
    /// The kind of closing bracket that was expected.
    expected: TokenKind,
    /// The kind of closing bracket that was encountered.
    found: TokenKind,
  },
  /// An opening bracket without its corresponding closing bracket was encountered.
  #[error("expected `{}` to close preceding `{}`", .expected, .expected.opener())]
  UnmatchedBracket {
    /// The kind of closing bracket that was expected.
    expected: TokenKind,
  },
}
