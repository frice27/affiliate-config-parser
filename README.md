#  Affiliate Config Parser

A **Rust parser** for affiliate marketing offer configurations.  
Reads structured offer files and converts them into Rust structs for validation, analysis, and automation.

---

## 📖 Brief Description

This project parses affiliate offer configuration files written in a **simple DSL**.  
Each file describes an affiliate offer, its GEO, traffic sources, payout, and conversion rate (CR).  
The parser validates the syntax, extracts data, and provides a Rust-native representation for further processing.

---

##  Example Offer File (`example.offer`)

OFFER: "Crypto Pro Max"
GEO: US, CA
TRAFFIC: Facebook, TikTok
PAYOUT: 42.5 USD
CR: 1.25%

Tips for formatting:
- Always use uppercase keywords: OFFER, GEO, TRAFFIC, PAYOUT, CR
- Use quotes for offer names with spaces
- Separate multiple GEO or TRAFFIC values with commas
- Always include units (USD for payout, % for CR)

 ## Grammar Rules
go
Config      := Rule+
Rule        := OfferRule | GeoRule | TrafficRule | PayoutRule | CRRule
OfferRule   := "OFFER:" <STRING>
GeoRule     := "GEO:" <IDENT_LIST>
TrafficRule := "TRAFFIC:" <IDENT_LIST>
PayoutRule  := "PAYOUT:" <NUMBER> "USD"
CRRule      := "CR:" <NUMBER> "%"
##  Diagram (Workflow)
pgsql
+----------------+       +----------------+
| Read file line | --->  | Match grammar  |
+----------------+       +----------------+
        |                      |
        v                      v
+----------------+       +----------------+
|  Parse value   | --->  | Store in struct|
+----------------+       +----------------+
## How it Works
The CLI reads an .offer file line by line.

Each line is matched against the grammar rules.

Values are stored in a Rust struct:

rust
pub struct OfferConfig {
    pub name: String,
    pub geo: Vec<String>,
    pub traffic: Vec<String>,
    pub payout: f32,
    pub cr: f32,
}
Errors are handled with thiserror in the library and anyhow in tests.

 ## Project Structure
bash
affiliate-config-parser/
├── Cargo.toml
├── README.md
├── Makefile
├── src/
│   ├── main.rs        # CLI interface
│   ├── lib.rs         # Library entry
│   └── parser.rs      # Parsing logic
└── tests/
    └── parser_tests.rs
 ## CLI Commands
bash
cargo run -- parse <file>  
# Parse an offer file
cargo run -- help         
# Show help
cargo run -- credits      
# Show credits
## Example usage:

bash
cargo run -- parse example.offer
 Unit Tests
Each grammar rule has at least one unit test.
Tests are located in tests/parser_tests.rs.
Errors are handled with anyhow.
Ensure all rules (OFFER, GEO, TRAFFIC, PAYOUT, CR) are tested.

 Requirements Coverage

✅ Project name included in Cargo.toml + README

✅ Brief description

✅ Technical parsing description

✅ 4+ grammar rules

✅ Unit tests

✅ lib.rs and main.rs

✅ CLI

✅ Error handling: thiserror + anyhow

✅ Diagram / grammar included

✅ Documentation for each rule

✅ Ready for crates.io release
