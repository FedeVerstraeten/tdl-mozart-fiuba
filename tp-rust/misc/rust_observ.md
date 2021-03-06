---
title: Rust - observaciones
author: Facultad de Ingeniería - UBA
geometry: left=2.5cm,right=2.5cm,margin=2cm,headheight=36pt
date: 07 de Octubre de 2018
fontsize: 12pt
rsize: "a4"
header-includes:
    - \usepackage{fancyhdr}
    - \pagestyle{fancy}
    - \usepackage{amsfonts, amsmath, amsthm, amssymb}
    - \usepackage{graphicx}
    - \usepackage{xcolor}
    - \usepackage{siunitx}
---

RUST: "safe, concurrent, practical language"
============================================

# CARGO

- *cargo.toml* generated by `cargo new`. TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.
- [package] is a section heading that indicates that the following statements are configuring a package. 
- [dependencies], is the start of a section for you to list any of your project’s dependencies.
- *Crates*: In Rust, packages of code are referred to as crates.
- [Crates.io](https://crates.io/) is where people in the Rust ecosystem post their open source Rust projects for others to use.
- Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers. The number 0.3.14 is actually shorthand for ^0.3.14, which means “any version that has a public API compatible with version 0.3.14.”
- *cargo.lock*: When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. Your project will remain at 0.3.14 (current version) until you explicitly upgrade.
- `cargo update` which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml.

# Crate

- A crate is a package of Rust code.
- The project we’ve been building is a binary crate, which is an executable.

# Random numbers

- Rust doesn’t yet include random number functionality in its standard library. However, the Rust team does provide a rand crate.
- The rand crate is a library crate, which contains code intended to be used in other programs.
- The `rand::thread_rng` function will give us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system.Next, we call the `gen_range` method on the random number generator. This method is defined by the Rng trait that we brought into scope with the `use rand::Rng` statement. 

# Traits

- You won’t just know which traits to use and which methods and functions to call from a crate. Instructions for using a crate are in each crate’s documentation. Another neat feature of Cargo is that you can run the `cargo doc --open` command, which will build documentation provided by all of your dependencies locally and open it in your browser.

# Match

- A match expression is made up of arms. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. The match construct and patterns are powerful features in Rust that let you express a variety of situations your code might encounter and make sure that you handle them all. 