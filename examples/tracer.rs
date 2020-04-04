use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use argh::{self, FromArgs};
use covidcotra::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct AuthorityDb {
    authority: Authority,
    infected: HashSet<HashedIdentity>,
    tainted: HashSet<HashedIdentity>,
}

impl Default for AuthorityDb {
    fn default() -> AuthorityDb {
        AuthorityDb {
            authority: Authority::unique(),
            infected: HashSet::new(),
            tainted: HashSet::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Me {
    identities: Vec<Identity>,
    contacts: ContactLog,
}

pub fn load<P: AsRef<Path>, D: DeserializeOwned + Default>(p: P) -> D {
    if fs::metadata(p.as_ref()).is_err() {
        D::default()
    } else {
        serde_json::from_slice(&fs::read(p.as_ref()).unwrap()).unwrap()
    }
}

pub fn save<P: AsRef<Path>, S: Serialize>(p: P, obj: &S) {
    let mut vec = serde_json::to_string_pretty(obj).unwrap();
    vec.push('\n');
    fs::write(p.as_ref(), vec).unwrap();
    eprintln!("Written to {}", p.as_ref().display());
}

/// Example app for covidcotra
#[derive(FromArgs, Debug)]
struct Cli {
    #[argh(subcommand)]
    cmd: Command,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
pub enum Command {
    CreateAuthority(CreateAuthorityCommand),
    CheckStatus(CheckStatusCommand),
    ImportInfected(ImportInfectedCommand),
    NewIdentity(NewIdentityCommand),
    NewShareIdentity(NewShareIdentityCommand),
    AddContact(AddContactCommand),
}

/// Creates a new authority.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "create-authority")]
pub struct CreateAuthorityCommand {
    /// path to authority file.
    #[argh(
        option,
        default = "env::current_dir().unwrap().join(\"authority.json\")"
    )]
    path: PathBuf,
}

/// Adds contacts as taints
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "import-infected")]
pub struct ImportInfectedCommand {
    /// path to authority file.
    #[argh(
        option,
        default = "env::current_dir().unwrap().join(\"authority.json\")"
    )]
    authority_path: PathBuf,
    /// path to the identity file.
    #[argh(option, default = "env::current_dir().unwrap().join(\"me.json\")")]
    identity_path: PathBuf,
}

/// Creates a new identity.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "new-identity")]
pub struct NewIdentityCommand {
    /// path to identity file.
    #[argh(option, default = "env::current_dir().unwrap().join(\"me.json\")")]
    path: PathBuf,
}

/// Checks the status
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "check-status")]
pub struct CheckStatusCommand {
    /// path to authority file.
    #[argh(
        option,
        default = "env::current_dir().unwrap().join(\"authority.json\")"
    )]
    authority_path: PathBuf,
    /// path to identity file.
    #[argh(option, default = "env::current_dir().unwrap().join(\"me.json\")")]
    path: PathBuf,
}

/// Creates a new share identity.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "new-share-identity")]
pub struct NewShareIdentityCommand {
    /// path to identity file.
    #[argh(option, default = "env::current_dir().unwrap().join(\"me.json\")")]
    path: PathBuf,
    /// the authority public key.
    #[argh(option)]
    public_key: String,
}

/// Adds a contact.
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "add-contact")]
pub struct AddContactCommand {
    /// path to identity file.
    #[argh(option, default = "env::current_dir().unwrap().join(\"me.json\")")]
    path: PathBuf,
    /// the identity to add
    #[argh(option)]
    share_id: String,
}

fn main() {
    let cli: Cli = argh::from_env();
    match cli.cmd {
        Command::CreateAuthority(subcmd) => {
            let db: AuthorityDb = load(&subcmd.path);
            save(&subcmd.path, &db);
            println!("Public Key: {}", db.authority.public_key());
        }
        Command::ImportInfected(subcmd) => {
            let mut db: AuthorityDb = load(&subcmd.authority_path);
            let user: Me = load(&subcmd.identity_path);
            for identity in user.identities {
                db.infected.insert(identity.hashed_id().clone());
            }
            for (contact, _) in user.contacts.decode(db.authority.secret_key()).unwrap() {
                db.tainted.insert(contact.hash());
            }
            save(&subcmd.authority_path, &db);
            println!("{}", db.authority.public_key());
        }
        Command::NewIdentity(subcmd) => {
            let mut me: Me = load(&subcmd.path);
            me.identities.push(Identity::unique());
            save(&subcmd.path, &me);
        }
        Command::CheckStatus(subcmd) => {
            let db: AuthorityDb = load(&subcmd.authority_path);
            let me: Me = load(&subcmd.path);
            let mut infected = false;
            let mut tainted = false;
            for identity in me.identities {
                if db.infected.contains(identity.hashed_id()) {
                    infected = true;
                    break;
                }
                if db.tainted.contains(identity.hashed_id()) {
                    tainted = true;
                    break;
                }
            }
            if infected {
                println!("You're infected");
            } else if tainted {
                println!("You're tainted");
            } else {
                println!("You're clear");
            }
        }
        Command::NewShareIdentity(subcmd) => {
            let public_key: PublicKey = subcmd.public_key.parse().unwrap();
            let mut me: Me = load(&subcmd.path);
            if me.identities.is_empty() {
                me.identities.push(Identity::unique());
                save(&subcmd.path, &me);
            }
            let share_id = me.identities[me.identities.len() - 1].new_share_id(&public_key);
            println!("{}", share_id);
        }
        Command::AddContact(subcmd) => {
            let mut me: Me = load(&subcmd.path);
            let identity: ShareIdentity = subcmd.share_id.parse().unwrap();
            me.contacts.add(&identity);
            save(&subcmd.path, &me);
        }
    }
}
