//! Implements the central authority.
use serde::{Deserialize, Serialize};

use crate::crypto::{gen_keypair, PublicKey, SecretKey};

/// Represents the central authority.
#[derive(Serialize, Deserialize)]
pub struct Authority {
    secret_key: SecretKey,
    public_key: PublicKey,
}

impl Authority {
    /// Creates a new authority.
    pub fn unique() -> Authority {
        let (public_key, secret_key) = gen_keypair();
        Authority {
            public_key,
            secret_key,
        }
    }

    /// Returns the secret key of the authority.
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    /// Returns the public key of the authority.
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}
