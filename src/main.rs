mod commands;
use std::{
    collections::HashMap,
    os::unix::process::CommandExt,
    path::PathBuf,
    process::{self, Stdio},
};

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

fn open_custom_editor(code_dir: &str, cmd: &str) -> () {
    let _code_process = process::Command::new(cmd)
        .current_dir(code_dir)
        .args(["."])
        .exec();
    if let Some(_) = _code_process.raw_os_error() {
        println!(
            "\x1b[31mError: running the command {}, ensure it exists in your path\x1b[0m",
            cmd
        );
    }
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
        Commands::Add(AddArgs { mut path, name }) => {
            if alias_to_dir.contains_key(name.as_str()) {
                println!(
                    "\x1b[33mAlias \x1b[32m{} \x1b[33malready exists \x1b[0m",
                    name
                );
            } else {
                if path.is_absolute() {
                    alias_to_dir.insert(name, path);
                } else {
                    match path.strip_prefix("./") {
                        Ok(p) => path = p.to_owned(),
                        _ => path = path,
                    }

                    let pwd_output = process::Command::new("pwd")
                        .output()
                        .expect("Could not run pwd");
                    let pwd = String::from_utf8_lossy(&pwd_output.stdout);

                    let full_path =
                        pwd.to_string().replace("\n", "") + "/" + path.to_str().unwrap();
                    alias_to_dir.insert(name, full_path.into());
                }
            }
        }
        Commands::Code(CodeArgs {
            name,
            code_editor,
            insider_editor,
        }) => match alias_to_dir.get(name.as_str()) {
            Some(dir) => {
                println!("Opening {}", dir.to_str().unwrap());
                if code_editor == false && insider_editor == false {
                    open_custom_editor(dir.to_str().unwrap(), "nvim")
                } else {
                    if code_editor == true {
                        open_custom_editor(dir.to_str().unwrap(), "code")
                    } else if insider_editor == true {
                        open_custom_editor(dir.to_str().unwrap(), "codei")
                    }
                }
            }
            None => {
                println!(
                    "\x1b[33mAlias \x1b[31m{}\x1b[33m does not exist \x1b[0m",
                    name
                );
            }
        },
        Commands::List => {
            let mut res = String::new();
            for (alias, path) in alias_to_dir.iter() {
                res += format!("\x1b[33m{}\x1b[0m\t{}\n", alias, path.to_str().unwrap()).as_str();
            }
            println!("{}", res);
        }
        Commands::Remove(RemoveArgs { name }) => {
            if alias_to_dir.contains_key(name.as_str()) {
                alias_to_dir.remove(name.as_str());
                println!("\x1b[33mAlias \x1b[32m{} \x1b[33mis removed \x1b[0m", name);
            } else {
                println!(
                    "\x1b[33mAlias \x1b[31m{}\x1b[33m does not exist \x1b[0m",
                    name
                );
            }
        }
    }

    let config_content = deserialize_alias_to_dir(alias_to_dir);
    save_dopls_config(config_path, &config_content)?;
    Ok(())
}
