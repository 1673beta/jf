use anyhow::Context;
use clap::{Parser, ValueEnum};
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "jf")]
#[command(about = "A simple JSON file manager", long_about = None)]
struct Cli {
    #[arg(name = "FILE")]
    input: PathBuf,

    #[arg(short, long, default_value_t = 2)]
    indent_size: usize,

		#[arg(short = 's', long, default_value_t = IndentStyle::Space)]
		indent_style: IndentStyle,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    compact: bool,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
enum IndentStyle {
    Space,
    Tab,
}

impl std::fmt::Display for IndentStyle {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				IndentStyle::Space => write!(f, "space"),
				IndentStyle::Tab => write!(f, "tab"),
			}
		}
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let json_str = fs::read_to_string(&cli.input)
        .with_context(|| format!("Failed to read file: {}", cli.input.display()))?;

    let json_value: Value = serde_json::from_str(&json_str).context("Failed to parse JSON")?;

    let formatted_json = if cli.compact {
        serde_json::to_string(&json_value).context("Failed to convert JSON to string")?
    } else {
        let indent_str = " ".repeat(cli.indent_size);
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_str.as_bytes());
        let mut buffer = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(&mut buffer, formatter);
        json_value
            .serialize(&mut ser)
            .context("Failed to serialize JSON")?;
        String::from_utf8(buffer).context("Failed to convert buffer to string")?
    };

    fs::write(&cli.input, formatted_json)
        .with_context(|| format!("Failed to write to file: {}", cli.input.display()))?;
    println!("Formatted JSON written to {}", cli.input.display());
    Ok(())
}
