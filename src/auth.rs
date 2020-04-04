//! Implements the authentication layer.
use std::borrow::Cow;

use derive_more::{Display, Error};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use serde::{de, ser, Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

use crate::crypto::{seal, unseal, PublicKey, SecretKey};

const SHARED_SALT: &[u8; 16] = b"nX\xdfu\x1au=\xd7\xe3d.\x1c\xb2\x11P\x0b";

/// Just the Unique ID detached from the authentication.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UniqueIdentity(Uuid);

impl UniqueIdentity {
    /// Creates a new unique identity.
    pub fn unique() -> UniqueIdentity {
        UniqueIdentity(Uuid::new_v4())
    }

    /// Hashes this unique identity
    pub fn hash(&self) -> HashedIdentity {
        let mut hashed_id = [0u8; 32];
        pbkdf2::<Hmac<Sha256>>(self.0.as_bytes(), SHARED_SALT, 50000, &mut hashed_id);
        HashedIdentity(hashed_id)
    }
}

/// Represents the unique identity of a person with auth info.
///
/// The identity is to kept private on the device and not shared with anyone
/// other than authorities when you are infected.  After it has every been
/// shared it must by cycled.
///
/// For sharing for subscription or cehck-in purposes the hashed identity
/// must be used instead.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "IdentitySmol")]
pub struct Identity {
    unique_id: UniqueIdentity,
    #[serde(skip_serializing)]
    hashed_id: HashedIdentity,
}

#[derive(Serialize, Deserialize)]
struct IdentitySmol {
    unique_id: UniqueIdentity,
}

impl From<IdentitySmol> for Identity {
    fn from(smol: IdentitySmol) -> Identity {
        Identity {
            unique_id: smol.unique_id,
            hashed_id: smol.unique_id.hash(),
        }
    }
}

/// A hashed identity is a derived version of the real identity.
///
/// This can be more freely shared with authorities for update purposes.  It's
/// derived via PBKDF2-HMAC-SHA256 on 10.000 iterations and a well known salt.
/// This is not ideal but it's a compromise.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashedIdentity([u8; 32]);

impl Serialize for HashedIdentity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serde_bytes::serialize(&self.0[..], serializer)
    }
}

impl<'de> Deserialize<'de> for HashedIdentity {
    fn deserialize<D>(deserializer: D) -> Result<HashedIdentity, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let bytes: Cow<'de, [u8]> = serde_bytes::deserialize(deserializer)?;
        if bytes.len() == 32 {
            let mut id = [0u8; 32];
            id.copy_from_slice(&bytes);
            Ok(HashedIdentity(id))
        } else {
            use serde::de::Error;
            Err(D::Error::custom("cannot deserialize hashed identity"))
        }
    }
}

/// An identity that can be shared with others.
///
/// This identity should be rotated once every few minutes.  It's an encrypted
/// version of the unique ID and sent to other devices.  Only the central
/// authority's key can decode the contained identity.
#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct ShareIdentity(#[serde(with = "serde_bytes")] Vec<u8>);

impl ShareIdentity {
    /// Reveals the unique identity behind a shared identity
    pub fn reveal(&self, secret_key: &SecretKey) -> Option<UniqueIdentity> {
        unseal(&self.0, secret_key)
            .and_then(|x| Uuid::from_slice(&x).ok())
            .map(UniqueIdentity)
    }
}

/// Error when an identity cannot be parsed.
#[derive(Debug, Display, Error)]
#[display(fmt = "Could not parse identity")]
pub struct ParseIdentityError;

/// Error when a hashed identity cannot be parsed.
#[derive(Debug, Display, Error)]
#[display(fmt = "Could not parse hashed identity")]
pub struct ParseHashedIdentityError;

impl Identity {
    /// Creates a new random identity.
    pub fn unique() -> Identity {
        let unique_id = UniqueIdentity::unique();
        Identity {
            unique_id,
            hashed_id: unique_id.hash(),
        }
    }

    /// Returns the internal unique identity.
    pub fn unique_id(&self) -> &UniqueIdentity {
        &self.unique_id
    }

    /// Returns the hashed identity.
    pub fn hashed_id(&self) -> &HashedIdentity {
        &self.hashed_id
    }

    /// Creates a new shareable identity.
    pub fn new_share_id(&self, public_key: &PublicKey) -> ShareIdentity {
        ShareIdentity(seal(self.unique_id.0.as_bytes(), public_key))
    }
}
