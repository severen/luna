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

//! This module contains types and functions for parsing Luna source code.

use crate::lexer::Lexer;

// TODO: Write an actual parser.
pub fn parse(input: &str) {
  let lexer = Lexer::new(strip_shebang(input));

  for token in lexer {
    println!("{} => {}", token.kind, token.lexeme);
  }
}

// TODO: Move this into a module containing program file abstractions.
/// Strip the shebang line from the given string if it is present.
pub fn strip_shebang(input: &str) -> &str {
  if input.starts_with("#!") {
    // The byte index of the first character after the shebang line.
    let i = input
      .find('\n')
      .map(|i| i + 1)
      .unwrap_or_else(|| input.len());
    &input[i..]
  } else {
    input
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
