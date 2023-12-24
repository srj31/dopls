use std::{collections::HashMap, os::unix::process::CommandExt, path::PathBuf, process};

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
    /// Adds files to dopls config
    Add(AddArgs),
    /// Open edtior for the aliased directory
    Code(CodeArgs),
}

#[derive(Args)]
struct AddArgs {
    /// Path to the directory to add
    path: PathBuf,

    /// Alias for accessing the directory
    name: String,
}

#[derive(Args)]
struct CodeArgs {
    /// Alias of the directory
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

fn open_nvim(code_dir: &str) -> () {
    let _nvim_process = process::Command::new("nvim")
        .current_dir(code_dir)
        .args(["."])
        .exec();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let (config_path, mut config_content) = load_dopls_config()?;
    let mut alias_to_dir = HashMap::<&str, PathBuf>::new();

    for line in config_content.lines() {
        if let Some((alias, path)) = line.split_once(':') {
            alias_to_dir.insert(alias, PathBuf::from(path));
        }
    }

    match args.command {
        Commands::Add(AddArgs { path, name }) => {
            if alias_to_dir.contains_key(name.as_str()) {
                println!(
                    "\x1b[33mAlias \x1b[32m{} \x1b[33malready exists \x1b[0m",
                    name
                );
            }
            config_content += format!("{}:{}\n", name, path.to_str().unwrap()).as_str();
        }
        Commands::Code(CodeArgs { name }) => {
            if let Some(dir) = alias_to_dir.get(name.as_str()) {
                println!("Opening {}", dir.to_str().unwrap());
                open_nvim(dir.to_str().unwrap());
            }
        }
    }

    save_dopls_config(config_path, &config_content)?;
    Ok(())
}
