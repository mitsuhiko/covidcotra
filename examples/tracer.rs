use std::env;
use std::fs;
use std::path::PathBuf;

use argh::{self, FromArgs};
use covidcotra::*;
use serde_json;

/// Example app for covidcotra
#[derive(FromArgs, Debug)]
struct Cli {
    #[argh(subcommand)]
    cmd: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    NewAuthority(NewAuthorityCommand),
}

/// Creates a new authority.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "new-authority")]
pub struct NewAuthorityCommand {
    /// path to authority file.
    #[argh(
        option,
        default = "env::current_dir().unwrap().join(\"authority.json\")"
    )]
    path: PathBuf,
}

fn main() {
    let cli: Cli = argh::from_env();
    match cli.cmd {
        Command::NewAuthority(subcmd) => {
            let authority = Authority::unique();
            fs::write(
                &subcmd.path,
                serde_json::to_string_pretty(&authority).unwrap(),
            )
            .unwrap();
        }
    }
}
