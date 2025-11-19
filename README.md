Author: Semchuk Vladyslav 
affiliate-config-parser

A parser for a simple affiliate marketing configuration language.
The parser supports GEO rules, traffic sources, daily caps, and payout definitions.
Technical Description

This project implements a hand-written parser for a custom DSL (domain-specific language) used in affiliate marketing to describe payout configurations.

What is parsed?

The parser reads configuration files with rules like:

GEO UA
SOURCE Facebook
CAP 100
PAYOUT 3.50


Each line follows one of the grammar rules.

Grammar

Below are the grammar rules implemented in the parser:

Config      := Rule+
Rule        := GeoRule | SourceRule | CapRule | PayoutRule
GeoRule     := "GEO" <IDENT>
SourceRule  := "SOURCE" <IDENT>
CapRule     := "CAP" <NUMBER>
PayoutRule  := "PAYOUT" <NUMBER>

How results are used?

Parsed rules are converted into an internal Rust struct:

GEO → saves the GEO value

SOURCE → saves traffic source name

CAP → saves integer daily cap

PAYOUT → saves float payout amount

This structure can then be used to:

validate campaign settings

generate affiliate configurations

send data to APIs

store rules in a database

 Project Structure
src/
 ├── lib.rs        # exposes parser API
 ├── parser.rs     # main parsing logic
 └── main.rs       # CLI interface

tests/
 └── parser_tests.rs

Cargo.toml
Makefile
README.md

 Unit Tests

All grammar rules have at least one test in tests/parser_tests.rs.
Tests use anyhow for error handling and ensure full rule coverage.

CLI

Program supports:

cargo run -- parse <file>
cargo run -- help
cargo run -- credits

 Requirements Coverage

✔ Project name included in Cargo.toml + README

✔ Brief description

✔ Technical parsing description

✔ 4 grammar rules

✔ Unit tests

✔ lib.rs and main.rs

✔ CLI

✔ Error handling: thiserror + anyhow

✔ Diagram / grammar included

✔ Documentation for each rule

✔ Ready for crates.io release
