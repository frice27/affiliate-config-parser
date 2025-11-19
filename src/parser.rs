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

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("OFFER:") {
            let value = line["OFFER:".len()..].trim();
            let value = value.trim_matches('"');
            if value.is_empty() {
                return Err(ParseError::InvalidFormat(line.to_string()));
            }
            name = Some(value.to_string());
        } else if line.starts_with("GEO:") {
            let value = line["GEO:".len()..].trim();
            geo = value
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        } else if line.starts_with("TRAFFIC:") {
            let value = line["TRAFFIC:".len()..].trim();
            traffic = value
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        } else if line.starts_with("PAYOUT:") {
            let value = line["PAYOUT:".len()..].trim();
            if !value.ends_with("USD") {
                return Err(ParseError::InvalidFormat(line.to_string()));
            }
            let number_str = value[..value.len()-3].trim();
            payout = Some(number_str.parse::<f32>().map_err(|_| ParseError::InvalidFormat(line.to_string()))?);
        } else if line.starts_with("CR:") {
            let value = line["CR:".len()..].trim();
            if !value.ends_with('%') {
                return Err(ParseError::InvalidFormat(line.to_string()));
            }
            let number_str = value[..value.len()-1].trim();
            cr = Some(number_str.parse::<f32>().map_err(|_| ParseError::InvalidFormat(line.to_string()))?);
        } else {
            return Err(ParseError::InvalidFormat(line.to_string()));
        }
    }

    // Check required fields
    let name = name.ok_or(ParseError::MissingField("OFFER".to_string()))?;
    let payout = payout.ok_or(ParseError::MissingField("PAYOUT".to_string()))?;
    let cr = cr.ok_or(ParseError::MissingField("CR".to_string()))?;

    Ok(OfferConfig {
        name,
        geo,
        traffic,
        payout,
        cr,
    })
}
