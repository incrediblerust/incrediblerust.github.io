use anyhow::{Context, Result};
use clap::{Arg, Command};
use std::path::Path;
use tokio::main;

mod config;
mod content;
mod generator;
mod templates;
mod utils;

use config::SiteConfig;
use generator::SiteGenerator;

#[main]
async fn main() -> Result<()> {
    let matches = Command::new("incrediblerust-generator")
        .version("0.1.0")
        .about("Custom static site generator for Incredible Rust")
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .value_name("DIR")
                .help("Source directory")
                .default_value("."),
        )
        .arg(
            Arg::new("destination")
                .short('d')
                .long("destination")
                .value_name("DIR")
                .help("Destination directory")
                .default_value("./_site"),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file")
                .default_value("_config.yml"),
        )
        .get_matches();

    let source = matches.get_one::<String>("source").unwrap();
    let destination = matches.get_one::<String>("destination").unwrap();
    let config_file = matches.get_one::<String>("config").unwrap();

    println!("🦀 Starting Incredible Rust Site Generator");
    println!("Source: {}", source);
    println!("Destination: {}", destination);

    // Load configuration
    let config_path = Path::new(source).join(config_file);
    let config = SiteConfig::load(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;

    // Create generator and build site
    let generator = SiteGenerator::new(source, destination, config)?;
    generator.build().await?;

    println!("✅ Site generated successfully!");
    Ok(())
}