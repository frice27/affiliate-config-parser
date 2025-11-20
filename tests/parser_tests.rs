use anyhow::Result;
use affiliate_config_parser::parse_offer_file;

#[test]
fn test_offer_rule() -> Result<()> {
    let content = r#"
OFFER: "Crypto Pro Max"
GEO: US
TRAFFIC: Facebook
PAYOUT: 50 USD
CR: 1.2%
"#;

    let path = "test_offer.offer";
    std::fs::write(path, content)?;

    let cfg = parse_offer_file(path)?;
    assert_eq!(cfg.name, "Crypto Pro Max");

    Ok(())
}

#[test]
fn test_geo_rule() -> Result<()> {
    let content = r#"
OFFER: "Test"
GEO: US, CA, UK
TRAFFIC: Google
PAYOUT: 20 USD
CR: 0.8%
"#;

    let path = "test_geo.offer";
    std::fs::write(path, content)?;

    let cfg = parse_offer_file(path)?;
    assert_eq!(cfg.geo, vec!["US", "CA", "UK"]);

    Ok(())
}

#[test]
fn test_traffic_rule() -> Result<()> {
    let content = r#"
OFFER: "TrafficTest"
GEO: US
TRAFFIC: Facebook, TikTok
PAYOUT: 15 USD
CR: 0.5%
"#;

    let path = "test_traffic.offer";
    std::fs::write(path, content)?;

    let cfg = parse_offer_file(path)?;
    assert_eq!(cfg.traffic, vec!["Facebook", "TikTok"]);

    Ok(())
}

#[test]
fn test_payout_rule() -> Result<()> {
    let content = r#"
OFFER: "PayTest"
GEO: US
TRAFFIC: Google
PAYOUT: 42.5 USD
CR: 1.3%
"#;

    let path = "test_payout.offer";
    std::fs::write(path, content)?;

    let cfg = parse_offer_file(path)?;
    assert!((cfg.payout - 42.5).abs() < f32::EPSILON);

    Ok(())
}

#[test]
fn test_cr_rule() -> Result<()> {
    let content = r#"
OFFER: "CRTest"
GEO: US
TRAFFIC: Google
PAYOUT: 10 USD
CR: 2.75%
"#;

    let path = "test_cr.offer";
    std::fs::write(path, content)?;

    let cfg = parse_offer_file(path)?;
    assert!((cfg.cr - 2.75).abs() < f32::EPSILON);

    Ok(())
}

#[test]
fn test_missing_offer_field() {
    let content = r#"
GEO: US
TRAFFIC: Google
PAYOUT: 10 USD
CR: 1%
"#;

    let path = "test_missing.offer";
    std::fs::write(path, content).unwrap();

    let result = parse_offer_file(path);
    assert!(result.is_err());
}
