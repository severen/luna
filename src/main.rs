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

use std::fs;

use anyhow::Result;
use structopt::StructOpt;
use rustyline::{Editor, error::ReadlineError};

mod lexer;
mod parser;

use crate::parser::parse;

#[derive(Debug, StructOpt)]
#[structopt(name = "luna", about = "A Lispy programming language.")]
struct Options {
  #[structopt(name = "FILE")]
  file_path: Option<String>,
}

fn main() -> Result<()> {
  let opts = Options::from_args();

  if let Some(path) = opts.file_path {
    let input = fs::read_to_string(path)?;
    parse(&input);
  } else {
    start_repl()?;
  }

  Ok(())
}

fn start_repl() -> Result<()> {
  let xdg_dirs = xdg::BaseDirectories::with_prefix("luna")?;
  let history_path = xdg_dirs.place_data_file("history.txt")?;

  let mut rl = Editor::<()>::new();
  if rl.load_history(&history_path).is_err() {
    println!("No previous history.");
  }

  loop {
    let line = rl.readline(">>> ");
    match line {
      Ok(line) => {
        rl.add_history_entry(line.as_str());

        parse(&line);
      },
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      },
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      },
      Err(error) => {
        println!("Error: {:?}", error);
        break;
      },
    }
  }

  rl.save_history(&history_path)?;

  Ok(())
}
