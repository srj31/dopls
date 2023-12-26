use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Adds files to dopls config
    Add(AddArgs),
    /// Open edtior for the aliased directory
    Code(CodeArgs),

    /// List all the aliases
    List,

    /// Remove an alias
    #[command(visible_alias = "rm")]
    Remove(RemoveArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// Alias for accessing the directory
    pub name: String,

    /// Path to the directory to add
    pub path: PathBuf,
}

#[derive(Args)]
pub struct CodeArgs {
    /// Alias of the directory
    pub name: String,

    /// Use code . command to open the editor
    #[arg(short, action = clap::ArgAction::SetTrue)]
    pub code_editor: Option<bool>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Alias of the directory to remove from the list
    pub name: String,
}
