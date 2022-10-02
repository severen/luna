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

use thiserror::Error;

pub mod lexer;
pub mod parser;

/// A byte position within an input stream.
pub type BytePos = usize;

/// A span of bytes within an input stream.
///
/// Specifically, a span is a range [a, b) for integers a and b such that a < b.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Span {
  /// The first byte in the range.
  pub start: BytePos,
  /// One byte past the end of the range.
  pub end: BytePos,
}

/// A syntax error.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Error)]
#[error("{kind}")]
pub struct Error {
  /// The span of text in which this syntax error is present.
  pub span: Span,
  /// The kind of this syntax error.
  pub kind: ErrorKind,
}

/// The specific kind of a syntax error.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Error)]
pub enum ErrorKind {
  /// An unexpected bracket.
  #[error("Unexpected bracket")]
  UnexpectedBracket,
  /// An unmatched bracket.
  #[error("Unmatched bracket")]
  UnmatchedBracket,
  /// An invalid token.
  #[error("Invalid token")]
  InvalidToken,
}
