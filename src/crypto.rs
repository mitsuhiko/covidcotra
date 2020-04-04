//! Internal crypto abstractions.
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use serde_plain::{forward_display_to_serde, forward_from_str_to_serde};
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as box_impl;
use sodiumoxide::crypto::sealedbox::curve25519blake2bxsalsa20poly1305 as sealbox_impl;

/// Represents a public key.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PublicKey(#[serde(with = "crate::utils::base64")] box_impl::PublicKey);

/// Error for invalid public keys.
#[derive(Debug, Error, Display, Clone)]
#[display(fmt = "cannot parse public key")]
pub struct PublicKeyParseError;

forward_display_to_serde!(PublicKey);
forward_from_str_to_serde!(PublicKey, |_x| -> PublicKeyParseError {
    PublicKeyParseError
});

/// Represents a secret key.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecretKey(#[serde(with = "crate::utils::base64")] box_impl::SecretKey);

/// Error for invalid secret keys.
#[derive(Debug, Error, Display, Clone)]
#[display(fmt = "cannot parse secret key")]
pub struct SecretKeyParseError;

forward_display_to_serde!(SecretKey);
forward_from_str_to_serde!(SecretKey, |_x| -> SecretKeyParseError {
    SecretKeyParseError
});

/// Generates a new key pair.
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = box_impl::gen_keypair();
    (PublicKey(pk), SecretKey(sk))
}

/// Encrypts some bytes.
pub(crate) fn seal(bytes: &[u8], receiver: &PublicKey) -> Vec<u8> {
    sealbox_impl::seal(bytes, &receiver.0)
}

/// Decrypts bytes.
pub(crate) fn unseal(bytes: &[u8], secret_key: &SecretKey) -> Option<Vec<u8>> {
    let public_key = secret_key.0.public_key();
    sealbox_impl::open(bytes, &public_key, &secret_key.0).ok()
}
