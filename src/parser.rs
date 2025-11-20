use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use thiserror::Error;

/// Represents an affiliate offer configuration parsed from a `.offer` file.
#[derive(Debug, Clone, PartialEq)]
pub struct OfferConfig {
    /// Name of the offer (OFFER rule)
    pub name: String,

    /// List of GEO codes (GEO rule)
    pub geo: Vec<String>,

    /// List of traffic sources (TRAFFIC rule)
    pub traffic: Vec<String>,

    /// Payout in USD (PAYOUT rule)
    pub payout: f32,

    /// Conversion rate in percent (CR rule)
    pub cr: f32,

    /// Optional daily cap (CAP rule)
    pub cap: Option<u32>,

    /// Optional vertical/category (VERTICAL rule)
    pub vertical: Option<String>,
}

/// Errors returned by the parser
#[derive(Error, Debug)]
pub enum ParseError {
    /// IO error while reading file
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    /// Invalid format for a line
    #[error("Invalid format in line: {0}")]
    InvalidFormat(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Duplicate field detected
    #[error("Duplicate field detected: {0}")]
    DuplicateField(String),

    /// Unknown rule/key found
    #[error("Unknown rule: {0}")]
    UnknownRule(String),

    /// Invalid number in line
    #[error("Invalid number in line: {0}")]
    InvalidNumber(String),

    /// Empty value for field
    #[error("Empty value for field: {0}")]
    EmptyValue(String),
}

/// Parse a single `.offer` file and return an OfferConfig struct
pub fn parse_offer_file<P: AsRef<Path>>(file_path: P) -> Result<OfferConfig, ParseError> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut name = None;
    let mut geo = None;
    let mut traffic = None;
    let mut payout = None;
    let mut cr = None;
    let mut cap = None;
    let mut vertical = None;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
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
            if geo.is_some() {
                return Err(ParseError::DuplicateField("GEO".to_string()));
            }
            let raw = line["GEO:".len()..].trim();
            if raw.is_empty() {
                return Err(ParseError::EmptyValue("GEO".to_string()));
            }
            let list: Vec<String> = raw
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();
            if list.iter().any(|v| v.is_empty()) {
                return Err(ParseError::EmptyValue("GEO".to_string()));
            }
            geo = Some(list);
        }

        // TRAFFIC
        else if line.starts_with("TRAFFIC:") {
            if traffic.is_some() {
                return Err(ParseError::DuplicateField("TRAFFIC".to_string()));
            }
            let raw = line["TRAFFIC:".len()..].trim();
            if raw.is_empty() {
                return Err(ParseError::EmptyValue("TRAFFIC".to_string()));
            }
            let list: Vec<String> = raw
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .collect();
            if list.iter().any(|v| v.is_empty()) {
                return Err(ParseError::EmptyValue("TRAFFIC".to_string()));
            }
            traffic = Some(list);
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
            let number = value[..value.len() - 3].trim();
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
            let number = value[..value.len() - 1].trim();
            cr = Some(number.parse::<f32>().map_err(|_| {
                ParseError::InvalidNumber(line.to_string())
            })?);
        }

        // CAP
        else if line.starts_with("CAP:") {
            if cap.is_some() {
                return Err(ParseError::DuplicateField("CAP".to_string()));
            }
            let raw = line["CAP:".len()..].trim();
            if raw.is_empty() {
                return Err(ParseError::EmptyValue("CAP".to_string()));
            }
            cap = Some(raw.parse::<u32>().map_err(|_| {
                ParseError::InvalidNumber(line.to_string())
            })?);
        }

        // VERTICAL
        else if line.starts_with("VERTICAL:") {
            if vertical.is_some() {
                return Err(ParseError::DuplicateField("VERTICAL".to_string()));
            }
            let raw = line["VERTICAL:".len()..].trim();
            if raw.is_empty() {
                return Err(ParseError::EmptyValue("VERTICAL".to_string()));
            }
            vertical = Some(raw.trim_matches('"').to_string());
        }

        // UNKNOWN RULE
        else {
            return Err(ParseError::UnknownRule(line.to_string()));
        }
    }

    Ok(OfferConfig {
        name: name.ok_or(ParseError::MissingField("OFFER".to_string()))?,
        geo: geo.unwrap_or_default(),
        traffic: traffic.unwrap_or_default(),
        payout: payout.ok_or(ParseError::MissingField("PAYOUT".to_string()))?,
        cr: cr.ok_or(ParseError::MissingField("CR".to_string()))?,
        cap,
        vertical,
    })
}
