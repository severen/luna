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

#![doc = include_str!("../README.md")]

use std::fs;

use anyhow::Result;
use clap::Parser;
use directories_next::ProjectDirs;
use rustyline::{error::ReadlineError, Editor};

mod syntax;

use crate::syntax::parser::parse;

/// Parsed command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
  #[arg(name = "FILE", help = "A path to a Scheme program to execute")]
  file_path: Option<String>,
}

fn main() -> Result<()> {
  let args = Args::parse();

  if let Some(path) = args.file_path {
    let input = fs::read_to_string(path)?;
    println!("{:?}", parse(&input));
  } else {
    repl()?;
  }

  Ok(())
}

fn repl() -> Result<()> {
  println!("Welcome to Luna v0.1.0!");
  println!("Press C-d to exit.");

  // The first and second parameters are respectively a reverse domain name and
  // organisation name, which are currently not used.
  let dirs = match ProjectDirs::from("", "", "luna") {
    Some(dirs) => dirs,
    // TODO: Handle a None value more gracefully by either throwing an error or disabling
    //       history.
    None => panic!("Could not find a valid $HOME path."),
  };
  // Ensure that the data directory exists to avoid errors when trying to write the
  // history file.
  if !dirs.data_dir().exists() {
    // TODO: Handle errors more gracefully.
    fs::create_dir(dirs.data_dir())?;
  }
  let history_path = dirs.data_dir().join("history.txt");

  let mut rl = Editor::<()>::new()?;
  if rl.load_history(&history_path).is_err() {
    println!("No previous history.");
  }

  loop {
    let line = rl.readline("> ");
    match line {
      Ok(line) => {
        rl.add_history_entry(&line);

        // TODO: Properly display and format syntax trees.
        match parse(&line) {
          Ok(sexpr) => println!("{:?}", sexpr),
          Err(error) => {
            println!("Syntax error: {}", error);
            println!("context: {}", &line[error.span.start..error.span.end]);
          },
        }
      },
      Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
      Err(error) => {
        println!("Error: {:?}", error);
        break;
      },
    }
  }

  rl.save_history(&history_path)?;

  Ok(())
}
