use clap::Parser;
use Jazz_linter::Linter;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version = "1.0", author = "Ibrahim Hamzat", about = "A JavaScript linter written in Rust")]
struct Opts {
    /// Input file to lint
    #[clap(parse(from_os_str))]
    input: PathBuf,

    /// Output format (text or json)
    #[clap(short, long, default_value = "text")]
    format: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let code = fs::read_to_string(&opts.input)?;
    let linter = Linter::new();
    let issues_json = linter.lint(&code);
    let issues: Vec<serde_json::Value> = serde_json::from_str(&issues_json)?;

    match opts.format.as_str() {
        "json" => println!("{}", issues_json),
        "text" | _ => {
            if issues.is_empty() {
                println!("No issues found.");
            } else {
                for (i, issue) in issues.iter().enumerate() {
                    println!("Issue {}:", i + 1);
                    println!("  Rule: {}", issue["rule"].as_str().unwrap_or("Unknown"));
                    println!("  Message: {}", issue["message"].as_str().unwrap_or("Unknown"));
                    println!();
                }
            }
        }
    }

    Ok(())
}