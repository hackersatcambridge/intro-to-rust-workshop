% Introduction to Rust
% Robin McCorkell
% 2017-11-09

# What is Rust?

* Modern C++-like language, version 1.0 released in 2015
* Designed for systems programming
* Guarantees memory safety
* Merges functional, imperative and object-oriented features
* Cross-platform, including the web
* "Most loved programming language" *

\* Stack Overflow Developer Survey 2016 and 2017

<div class="notes">
* Started by Mozilla
* Now in use by a growing number of companies
* Often compared to Golang, a Google language, but have different goals
</div>

# Getting Started

## Installing rustup

[www.rustup.rs](https://www.rustup.rs)

* On Windows, run the installer, then open a command prompt or PowerShell prompt.
* On Mac, run the shell command in a terminal, then open a new terminal.
* On Linux, install a distro package or run the shell command, then open a new
terminal.

To test, run: `rustc --version`

<div class="notes">
If broken, `rustup install toolchain stable`

* Nightly vs stable
* rustup allows installing different toolchain versions
* E.g. specific project using special version of nightly (toolchain override)
</div>

## The Build System

![crates.io](img/crates.io.png)

<div class="notes">
Cargo is the de-facto build system, handling dependency management, project
compilation, external build scripts, publishing releases (to crates.io), and
more

* Use Cargo to create a new project
* All config in Cargo.toml, very simple INI-like syntax (TOML)
* Strong integration with crates.io, community resource for Rust libraries (crates)

Basic commands, write on whiteboard:
* `cargo init [--bin]` (defaults to library)
* `cargo run`
* `cargo search <query>`
* `cargo test`
</div>

# Starting Simple

## Let's Make a Program

We're going to write a simple logging object.

![What we should see](img/simple-prog.png)

<div class="notes">
Go to live example

1) Write Logger trait, `fn log(&self, message: &str)`
2) LogPrinter struct contains a u32 message ID
3) Run, see immutability error
4) Fix error with `Cell<u32>`
</div>

## Objects

* Interfaces defined as traits, that can be implemented by other types
* To make a method, need to explicitly include a `self` parameter
* `self` will own the object (i.e. move it)
* `&self` will borrow the object (immutable)
* `&mut self` will mutably borrow the object
* We can hide inner mutability with a `Cell`

<div class="notes">
* `fn log()` borrows the logger object, and takes an immutable string
* Returns nothing
* Special syntax for self/this: elided from `self: &Self`

* Implement 'constructor', just another method
* Need a return type of the object (could also use `Self`)
* Automatic return if last semicolon omitted

* Don't really want to make `log()` take a mutable object, non-sensical
* Message ID is an implementation detail
* Use `Cell` to hide the mutable implementation detail
* `use std::cell::Cell`, then `Cell::new(v)`, `cell.get()` and `cell.set(v)`
* Rust ensures that data is owned in one place only
</div>

## Strings

* We used `&str` to borrow a string
* String literals like `"Hello, world!"` are actually statically allocated
* Static objects cannot be modified and can only be read by immutable reference, just like borrowing!
* We could have taken an owned `String`, but that causes allocations
* The `println!()` macro takes a format string and some number of args

<div class="notes">
* Reference to `str`, i.e. immutable string
* Contrast with owned `String`
* Use `println!()` macro, similar to other languages
* Macros deliniated by the bang after the identifier
</div>

# Something a Bit More Advanced

## Let's write a web server!

![Rust makes web servers easy](img/web-server.png)

<div class="notes">
NEED NIGHTLY

`rustup override set nightly` in new project directory, also `rustup update`

Go to live example

1) Go to Rocket HTTP docs
2) Basic hello world example with Rocket
3) Create `/hello/<name>` endpoint with `String`
4) Use match on name to add special message
5) Create `/fizzbuzz/<n>` endpoint with u32
6) Manually parse u32 from `String`, using unwrap
7) Switch unwrap for matching
8) Extend fizzbuzz with iterators to print all from 1 to n
</div>

## External Libraries (Crates)

* Download from crates.io
* Cargo will handle pulling dependencies matching the versions you want, and compiling them
* Link them into your project with `extern crate`
* Some crates need Rust nightly, most work with stable Rust

<div class="notes">
* Can use `cargo search` to search for crates
</div>

## Matching

* Match statements can deconstruct types
* Can match on actual values (like strings), on enums (like `Result`), and others
* Sometimes need to be careful with ownership

<div class="notes">
* Be careful matching with constants, a mis-typed constant name will just be a variable that will still match
* Can put conditionals in the match statement, i.e. match a branch if the parameter satisfies a test
* If a match uses an owned value, the ownership is moved into the taken branch
* If a match uses a borrowed value, the branches must use borrowed values also
* Matches are just normal statements, so can return values
</div>

## Error Handling

* Use `.unwrap()` on a `Result` to get the value
* But this will panic (crash) if it is actually an error!

![Unwrap panic](img/panic.png)

<div class="notes">
* String parsing can fail
* Other possible failures: file I/O, network requests, division (by zero), ...
* Match where possible
* Unwrap is often considered bad, use `.expect()` for better error 
</div>

## Iterators

* Rust supports functional-style iterators
* Iterators are lazy by default
* But traditional for loops are still available!

<div class="notes">
* Iterators are not the best solution to every problem (but are for many)
* Can create iterator from vectors (owned lists) or slices (borrowed lists)
* Or can create iterators as generators, without a list behind them
</div>

# Where Do We Go From Here?

## Rust Documentation

![The Rust API documentation](img/docs.png)

<div class="notes">
* Cargo can generate docs from your code, enhanced with doc comments
* Every crate on crates.io has docs, as does the standard library
* Popular crates include examples in the docs
</div>

## The Rust Book

![The Rust Book](img/book.png)

<div class="notes">
* Brilliant resource for learning Rust, or just as a reminder of concepts
* Covers every part of Rust in detail
* Includes gotchas and other useful information
* Currently being rewritten to be even more awesome
</div>

## Rust by Example

![Rust by Example](img/rust-by-example.png)

<div class="notes">
* Examples of many Rust concepts, complements the book
</div>

# Questions?

<div class="notes">
"Rust makes writing data structures hard, but concurrency easy"
</div>

