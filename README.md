# Luna

Luna is a work-in-progress hobby implementation of an interpreter for the Scheme
programming language.

Why write this, you ask? Well, for starters, the world _clearly_ needed yet another
implementation of Scheme. Moreover, programming languages are fun and interesting! In
implementing a well-designed, actually useful programming language, this project serves
as a vessel for learning about programming language theory, design, and implementation.
In particular, the goal is to implement Scheme as defined in
[R⁷RS](https://github.com/johnwcowan/r7rs-spec/blob/errata/spec/r7rs.pdf).

## Building

Luna is written in [Rust](https://rust-lang.org/) and hence it uses Cargo as
its build system. Once a Rust distribution is installed, Luna can be built with
the `cargo build` command, and run with the `cargo run` command. To build with
release optimisations enabled, pass the `--release` flag to either command.
