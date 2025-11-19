//! # affiliate-config-parser
//!
//! Library for parsing affiliate offer configuration files.
//! Provides functions to read `.offer` files and convert them into Rust structs.

pub mod parser;

pub use parser::{OfferConfig, ParseError, parse_offer_file};
