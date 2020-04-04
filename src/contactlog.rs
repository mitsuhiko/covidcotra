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
    pub fn decode(&self, secret_key: &SecretKey) -> Option<Vec<(UniqueIdentity, DateTime<Utc>)>> {
        let mut rv = vec![];
        for (contact, &timestamp) in self.seen.iter() {
            rv.push((contact.reveal(secret_key)?, timestamp));
        }
        Some(rv)
    }
}

impl Default for ContactLog {
    fn default() -> ContactLog {
        ContactLog::new()
    }
}
