use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use thiserror::Error;

/// Struct representing an affiliate offer configuration
#[derive(Debug, Clone, PartialEq)]
pub struct OfferConfig {
    pub name: String,
    pub geo: Vec<String>,
    pub traffic: Vec<String>,
    pub payout: f32,
    pub cr: f32,
    pub cap: Option<u32>,       // NEW RULE
    pub vertical: Option<String>, // NEW RULE
}

/// Errors returned by the parser
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid format in line: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Duplicate field detected: {0}")]
    DuplicateField(String),

    #[error("Unknown rule: {0}")]
    UnknownRule(String),

    #[error("Invalid number in line: {0}")]
    InvalidNumber(String),

    #[error("Empty value for field: {0}")]
    EmptyValue(String),
}

/// Parse a single `.offer` file and return an OfferConfig struct
pub fn parse_offer_file<P: AsRef<Path>>(file_path: P) -> Result<OfferConfig, ParseError> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut name = None;
    let mut geo: Vec<String> = Vec::new();
    let mut traffic: Vec<String> = Vec::new();
    let mut payout = None;
    let mut cr = None;
    let mut cap = None;
    let mut vertical = None;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // OFFER
        if line.starts_with("OFFER:") {
            if name.is_some() {
                return Err(ParseError::DuplicateField("OFFER".to_string()));
            }
            let value = line["OFFER:".len()..].trim().trim_matches('"');
            if value.is_empty() {
                return Err(ParseError::EmptyValue("OFFER".to_string()));
            }
            name = Some(value.to_string());
        }

        // GEO
        else if line.starts_with("GEO:") {
            let value = line["GEO:".len()..].trim();
            if value.is_empty() {
                return Err(ParseError::EmptyValue("GEO".to_string()));
            }
            geo = value.split(',').map(|s| s.trim().to_string()).collect();
        }

        // TRAFFIC
        else if line.starts_with("TRAFFIC:") {
            let value = line["TRAFFIC:".len()..].trim();
            if value.is_empty() {
                return Err(ParseError::EmptyValue("TRAFFIC".to_string()));
            }
            traffic = value.split(',').map(|s| s.trim().to_string()).collect();
        }

        // PAYOUT
        else if line.starts_with("PAYOUT:") {
            if payout.is_some() {
                return Err(ParseError::DuplicateField("PAYOUT".to_string()));
            }
            let value = line["PAYOUT:".len()..].trim();
            if !value.ends_with("USD") {
                return Err(ParseError::InvalidFormat(line.to_string()));
            }
            let number = value[..value.len()-3].trim();
            payout = Some(number.parse::<f32>().map_err(|_| {
                ParseError::InvalidNumber(line.to_string())
            })?);
        }

        // CR
        else if line.starts_with("CR:") {
            if cr.is_some() {
                return Err(ParseError::DuplicateField("CR".to_string()));
            }
            let value = line["CR:".len()..].trim();
            if !value.ends_with('%') {
                return Err(ParseError::InvalidFormat(line.to_string()));
            }
            let number = value[..value.len()-1].trim();
            cr = Some(number.parse::<f32>().map_err(|_| {
                ParseError::InvalidNumber(line.to_string())
            })?);
        }

        // CAP (NEW RULE)
        else if line.starts_with("CAP:") {
            if cap.is_some() {
                return Err(ParseError::DuplicateField("CAP".to_string()));
            }
            let value = line["CAP:".len()..].trim();
            cap = Some(value.parse::<u32>().map_err(|_| {
                ParseError::InvalidNumber(line.to_string())
            })?);
        }

        // VERTICAL (NEW RULE)
        else if line.starts_with("VERTICAL:") {
            if vertical.is_some() {
                return Err(ParseError::DuplicateField("VERTICAL".to_string()));
            }
            let value = line["VERTICAL:".len()..].trim();
            if value.is_empty() {
                return Err(ParseError::EmptyValue("VERTICAL".to_string()));
            }
            vertical = Some(value.to_string());
        }

        // UNKNOWN RULE
        else {
            return Err(ParseError::UnknownRule(line.to_string()));
        }
    }

    Ok(OfferConfig {
        name: name.ok_or(ParseError::MissingField("OFFER".to_string()))?,
        geo,
        traffic,
        payout: payout.ok_or(ParseError::MissingField("PAYOUT".to_string()))?,
        cr: cr.ok_or(ParseError::MissingField("CR".to_string()))?,
        cap,
        vertical,
    })
}
