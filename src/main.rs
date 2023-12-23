use std::{fs, io::Write, path::PathBuf};

use clap::{Args, Parser, Subcommand};
extern crate dirs;

/// Simple program to open your favourite projects in your editor of choice
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
    /// Path to the directory to add
    path: PathBuf,

    /// Unique name for accessing the directory
    name: String,
}

fn load_dopls_config() -> Result<(PathBuf, String), Box<dyn std::error::Error>> {
    let mut config_path = dirs::config_dir().unwrap();
    config_path.push("dopls");
    config_path.push(".config");
    let config_content = if let Ok(content) = std::fs::read(&config_path) {
        String::from_utf8_lossy(&content).to_string()
    } else {
        "".to_owned()
    };
    Ok((config_path, config_content))
}

fn save_dopls_config(
    config_path: PathBuf,
    config_content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir_name = std::path::Path::new(&config_path)
        .parent()
        .ok_or("Failed to get parent directory ")?;

    std::fs::create_dir_all(dir_name)?;
    std::fs::write(&config_path, config_content)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let (config_path, mut config_content) = load_dopls_config()?;

    match args.command {
        Commands::Add(AddArgs { path, name }) => {
            config_content += format!("\"{}\": \"{}\"\n", name, path.to_str().unwrap()).as_str();
        }
    }

    save_dopls_config(config_path, &config_content)?;
    Ok(())
}
