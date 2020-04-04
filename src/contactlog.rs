//! Implements the contact log.
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::auth::{ShareIdentity, UniqueIdentity};
use crate::crypto::SecretKey;

/// Represents contacts observed recently.
#[derive(Serialize, Deserialize)]
pub struct ContactLog {
    seen: HashMap<ShareIdentity, DateTime<Utc>>,
}

impl ContactLog {
    /// Creates an empty contact log.
    pub fn new() -> ContactLog {
        ContactLog {
            seen: HashMap::new(),
        }
    }

    /// Registers a contact and the current timestamp.
    pub fn add(&mut self, share_id: &ShareIdentity) {
        self.seen.insert(share_id.clone(), Utc::now());
    }

    /// Decodes the contacts with the secret key of the authority.
    ///
    /// This returns `None` if decoding fails (invalid key or data).
    pub fn decode(&self, secret_key: &SecretKey) -> Option<Vec<(UniqueIdentity, DateTime<Utc>)>> {
        let mut rv = HashMap::new();
        for (contact, &timestamp) in self.seen.iter() {
            let unique_id = contact.reveal(secret_key)?;
            if rv.get(&unique_id).map_or(true, |old| *old < timestamp) {
                rv.insert(unique_id, timestamp);
            }
        }
        Some(rv.into_iter().collect())
    }
}

impl Default for ContactLog {
    fn default() -> ContactLog {
        ContactLog::new()
    }
}
