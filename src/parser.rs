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

//! Types and functions for parsing Luna source code.
//!
//! The formal grammar for Luna is specified by the following EBNF:
//!
//! ```
//! <program> -> <datum>*
//! <datum> -> <atom> | <list>
//! <atom> -> <symbol> | <int> | <bool>
//! <list> -> (<datum>*) | [<datum>*] | {<datum>*}
//! ```
//!
//! Note that the nonterminals `<datum>` and `<atom>` are only present in the
//! EBNF for ease of discussion and reading; they do not exist in the `Ast`
//! enum.

use crate::lexer::{Lexer, TokenKind, get_matching};

#[derive(Debug)]
/// An abstract syntax tree of Luna source code.
pub enum Ast {
  Program(Vec<Ast>),
  Symbol(String),
  Int(i32),
  Bool(bool),
  List(Vec<Ast>),
}

/// Parse the given source code into an abstract syntax tree.
///
/// This amounts to parsing the `<program>` nonterminal.
pub fn parse(input: &str) -> Ast {
  let mut lexer = Lexer::new(strip_shebang(input));

  let mut program = Vec::new();
  while let Some(token) = lexer.peek() {
    use TokenKind::*;

    program.push(match token.kind {
      Symbol => parse_symbol(&mut lexer),
      Int => parse_int(&mut lexer),
      Bool => parse_bool(&mut lexer),
      LParen | LBracket | LBrace => parse_list(&mut lexer),
      _ => unimplemented!(),
    })
  }

  Ast::Program(program)
}

/// Parse the `<symbol>` terminal.
fn parse_symbol(lexer: &mut Lexer) -> Ast {
    Ast::Symbol(lexer.next().unwrap().lexeme.to_string())
}

/// Parse the `<int>` terminal.
fn parse_int(lexer: &mut Lexer) -> Ast {
  Ast::Int(lexer.next().unwrap().lexeme.parse().unwrap())
}

/// Parse the `<bool>` terminal.
fn parse_bool(lexer: &mut Lexer) -> Ast {
  let lexeme = lexer.next().unwrap().lexeme;
  let value = match lexeme {
    "#t" | "#T" => true,
    "#f" | "#F" => false,
    _ => unreachable!(),
  };

  Ast::Bool(value)
}

/// Parse the `<list>` nonterminal.
fn parse_list(lexer: &mut Lexer) -> Ast {
  let mut list = Vec::new();

  let opener = lexer.next().unwrap(); // Consume the opening bracket.
  while let Some(token) = lexer.peek() {
    use TokenKind::*;

    list.push(match token.kind {
      Symbol => parse_symbol(lexer),
      Int => parse_int(lexer),
      Bool => parse_bool(lexer),
      LParen | LBracket | LBrace => parse_list(lexer),
      RParen | RBracket | RBrace => {
        // TODO: Handle this more gracefully.
        assert_eq!(token.kind, get_matching(opener.kind));
        break;
      },
      _ => unimplemented!(),
    })
  }
  // TODO: Be more robust when it comes to handling a list with only an opening
  //       bracket.
  lexer.next().unwrap(); // Consume the closing bracket.

  Ast::List(list)
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
  fn parse_program() {
    parse("(defn fac [n]\n(fac (minus n 1)))\n\n(print (fac 5))");
  }

  #[test]
  fn parse_symbol() {
    parse("hello");
    parse("foo bar");
    parse("foo\nbar");
  }

  #[test]
  fn parse_int() {
    parse("10");
    parse("0 11");
    parse("0 -11");
  }

  #[test]
  fn parse_bool() {
    parse("#t #T");
    parse("#f #F");
  }

  #[test]
  fn parse_list() {
    // Can we parse empty lists?
    parse("()");
    parse("[]");
    parse("{}");

    // Can we parse normal lists?
    parse("(1 2 3)");
    parse("[1 2 3]");
    parse("{1 2 3}");

    // Can we parse nested lists?
    parse("(1 [2 {3}])");
    parse("{1 [2 3]}");
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
