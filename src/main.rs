mod commands;
use std::{collections::HashMap, os::unix::process::CommandExt, path::PathBuf, process};

use clap::Parser;
extern crate dirs;

use commands::commands::{AddArgs, CodeArgs, Commands, RemoveArgs};

/// Simple program to open your favourite projects in your editor of choice
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

fn deserialize_alias_to_dir(alias_to_dir: HashMap<String, PathBuf>) -> String {
    let mut config_content = String::new();
    for (alias, path) in alias_to_dir {
        config_content += format!("{}:{}\n", alias, path.to_str().unwrap()).as_str();
    }
    config_content
}

fn open_nvim(code_dir: &str) -> () {
    let _nvim_process = process::Command::new("nvim")
        .current_dir(code_dir)
        .args(["."])
        .exec();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let (config_path, config_content) = load_dopls_config()?;
    let mut alias_to_dir = HashMap::<String, PathBuf>::new();

    for line in config_content.lines() {
        if let Some((alias, path)) = line.split_once(':') {
            alias_to_dir.insert(alias.to_owned(), PathBuf::from(path));
        }
    }

    match args.command {
        Commands::Add(AddArgs { path, name }) => {
            if alias_to_dir.contains_key(name.as_str()) {
                println!(
                    "\x1b[33mAlias \x1b[32m{} \x1b[33malready exists \x1b[0m",
                    name
                );
            } else {
                alias_to_dir.insert(name, path);
            }
        }
        Commands::Code(CodeArgs { name }) => {
            if let Some(dir) = alias_to_dir.get(name.as_str()) {
                println!("Opening {}", dir.to_str().unwrap());
                open_nvim(dir.to_str().unwrap());
            }
        }
        Commands::List => {
            let mut res = String::new();
            for (alias, path) in alias_to_dir.iter() {
                res += format!("\x1b[33m{}\x1b[0m\t{}\n", alias, path.to_str().unwrap()).as_str();
            }
            println!("{}", res);
        }
        Commands::Remove(RemoveArgs { name }) => {
            alias_to_dir.remove(name.as_str());
        }
    }

    let config_content = deserialize_alias_to_dir(alias_to_dir);
    save_dopls_config(config_path, &config_content)?;
    Ok(())
}
