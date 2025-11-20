use anyhow::Result;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use affiliate_config_parser::parse_offer_file;

/// Helper: write given content to a unique temporary file in repo root and return its path.
fn write_tmp(content: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = format!("test_{}.offer", nanos);
    fs::write(&path, content).expect("failed to write test file");
    path
}

/// Helper: remove file if exists (best-effort cleanup)
fn cleanup(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_valid_full_offer() -> Result<()> {
    let content = r#"
OFFER: "Crypto Pro Max"
GEO: US, CA
TRAFFIC: Facebook, TikTok
PAYOUT: 42.5 USD
CR: 1.25%
CAP: 200
VERTICAL: Crypto
"#;
    let path = write_tmp(content);
    let cfg = parse_offer_file(&path)?;
    assert_eq!(cfg.name, "Crypto Pro Max");
    assert_eq!(cfg.geo, vec!["US", "CA"]);
    assert_eq!(cfg.traffic, vec!["Facebook", "TikTok"]);
    assert!((cfg.payout - 42.5).abs() < f32::EPSILON);
    assert!((cfg.cr - 1.25).abs() < f32::EPSILON);
    assert_eq!(cfg.cap, Some(200));
    assert_eq!(cfg.vertical.as_deref(), Some("Crypto"));
    cleanup(&path);
    Ok(())
}

#[test]
fn test_offer_without_optional_fields() -> Result<()> {
    let content = r#"
OFFER: "NoOptional"
GEO: US
TRAFFIC: Facebook
PAYOUT: 10 USD
CR: 0.5%
"#;
    let path = write_tmp(content);
    let cfg = parse_offer_file(&path)?;
    assert_eq!(cfg.name, "NoOptional");
    assert_eq!(cfg.geo, vec!["US"]);
    assert_eq!(cfg.traffic, vec!["Facebook"]);
    assert_eq!(cfg.cap, None);
    assert_eq!(cfg.vertical, None);
    cleanup(&path);
    Ok(())
}

#[test]
fn test_geo_list_trimming_and_quotes() -> Result<()> {
    let content = r#"
OFFER: "QTest"
GEO: US , "CA" ,  FI
TRAFFIC: "Google UAC", Facebook
PAYOUT: 5 USD
CR: 0.25%
"#;
    let path = write_tmp(content);
    let cfg = parse_offer_file(&path)?;
    assert_eq!(cfg.geo, vec!["US", "CA", "FI"]);
    assert_eq!(cfg.traffic, vec!["Google UAC", "Facebook"]);
    cleanup(&path);
    Ok(())
}

#[test]
fn test_duplicate_field_offer() {
    let content = r#"
OFFER: "A"
OFFER: "B"
GEO: US
TRAFFIC: FB
PAYOUT: 1 USD
CR: 1%
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    let err = res.unwrap_err().to_string();
    assert!(err.contains("Duplicate"));
    cleanup(&path);
}

#[test]
fn test_missing_required_field_offer() {
    let content = r#"
GEO: US
TRAFFIC: Google
PAYOUT: 10 USD
CR: 1%
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    let err = res.unwrap_err().to_string();
    assert!(err.contains("Missing"));
    cleanup(&path);
}

#[test]
fn test_invalid_payout_without_usd() {
    let content = r#"
OFFER: "BadPayout"
GEO: US
TRAFFIC: FB
PAYOUT: 42.5
CR: 1%
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    cleanup(&path);
}

#[test]
fn test_invalid_cr_without_percent() {
    let content = r#"
OFFER: "BadCR"
GEO: US
TRAFFIC: FB
PAYOUT: 10 USD
CR: 1.0
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    cleanup(&path);
}

#[test]
fn test_invalid_cap_not_number() {
    let content = r#"
OFFER: "BadCap"
GEO: US
TRAFFIC: FB
PAYOUT: 10 USD
CR: 1%
CAP: not_a_number
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    cleanup(&path);
}

#[test]
fn test_empty_geo_list_error() {
    let content = r#"
OFFER: "EmptyGeo"
GEO: , ,
TRAFFIC: FB
PAYOUT: 10 USD
CR: 1%
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    cleanup(&path);
}

#[test]
fn test_unknown_rule_error() {
    let content = r#"
OFFER: "UnknownRule"
SOMETHING: x
GEO: US
TRAFFIC: FB
PAYOUT: 10 USD
CR: 1%
"#;
    let path = write_tmp(content);
    let res = parse_offer_file(&path);
    assert!(res.is_err());
    cleanup(&path);
}

#[test]
fn test_quoted_offer_with_commas_in_traffic() -> Result<()> {
    let content = r#"
OFFER: "Comma Offer"
GEO: US
TRAFFIC: "Google, Search", Facebook
PAYOUT: 1 USD
CR: 0.1%
"#;
    let path = write_tmp(content);
    let cfg = parse_offer_file(&path)?;
    assert_eq!(cfg.traffic, vec!["Google, Search", "Facebook"]);
    cleanup(&path);
    Ok(())
}

#[test]
fn test_comments_and_empty_lines() -> Result<()> {
    let content = r#"
# This is a comment
OFFER: "Commented"
GEO: US

# another comment
TRAFFIC: FB
PAYOUT: 1 USD
CR: 0.1%
"#;
    let path = write_tmp(content);
    let cfg = parse_offer_file(&path)?;
    assert_eq!(cfg.name, "Commented");
    assert_eq!(cfg.geo, vec!["US"]);
    assert_eq!(cfg.traffic, vec!["FB"]);
    cleanup(&path);
    Ok(())
}
