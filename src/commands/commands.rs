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
    Remove(RemoveArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// Path to the directory to add
    pub path: PathBuf,

    /// Alias for accessing the directory
    pub name: String,
}

#[derive(Args)]
pub struct CodeArgs {
    /// Alias of the directory
    pub name: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Alias of the directory to remove from the list
    pub name: String,
}
