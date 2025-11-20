use affiliate_config_parser::parse_offer_file;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: missing file path.\n");
                print_help();
                return;
            }

            let file_path = &args[2];

            match parse_offer_file(file_path) {
                Ok(config) => {
                    println!("Parsed offer:");
                    println!("Name: {}", config.name);
                    println!("GEO: {:?}", config.geo);
                    println!("Traffic: {:?}", config.traffic);
                    println!("Payout: {} USD", config.payout);
                    println!("CR: {}%", config.cr);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }

        "help" => print_help(),

        "credits" => {
            println!("affiliate-config-parser v0.1.0");
            println!("Developed by Vlad");
        }

        _ => {
            eprintln!("Unknown command.\n");
            print_help();
        }
    }
}

fn print_help() {
    println!("affiliate-config-parser — CLI tool");
    println!();
    println!("Commands:");
    println!("  parse <file>    Parse an .offer file");
    println!("  help            Show help message");
    println!("  credits         Show project credits");
}
