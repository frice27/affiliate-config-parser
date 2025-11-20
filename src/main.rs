use affiliate_config_parser::parse_offer_file;
use std::env;

/// Simple CLI interface for the affiliate-config-parser project
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: No file provided.");
                print_help();
                return;
            }
            let file = &args[2];
            match parse_offer_file(file) {
                Ok(config) => {
                    println!("Parsed successfully:\n{:#?}", config);
                }
                Err(e) => {
                    eprintln!("Error parsing file: {}", e);
                }
            }
        }

        "help" => print_help(),

        "credits" => {
            println!("affiliate-config-parser v0.1.0");
            println!("Created by Vlad for educational purposes.");
            println!("GitHub repository: https://github.com/your-username/affiliate-config-parser");
        }

        _ => {
            eprintln!("Unknown command.");
            print_help();
        }
    }
}

fn print_help() {
    println!("affiliate-config-parser CLI");
    println!();
    println!("Commands:");
    println!("  parse <file>     Parse an .offer configuration file");
    println!("  help             Show available commands");
    println!("  credits          Show authorship information");
}
