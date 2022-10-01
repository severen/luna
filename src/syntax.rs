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

use std::ops::Range;

use thiserror::Error;

pub mod lexer;
pub mod parser;

/// A span of text represented by an inclusive start byte index and an exclusive end byte
/// index.
pub type Span = Range<usize>;

/// A syntax error.
#[derive(Debug, Error)]
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
