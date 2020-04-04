//! Internal crypto abstractions.
use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305 as box_impl;
use sodiumoxide::crypto::sealedbox::curve25519blake2bxsalsa20poly1305 as sealbox_impl;

/// Represents a public key.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PublicKey(box_impl::PublicKey);

/// Represents a secret key.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SecretKey(box_impl::SecretKey);

/// Generates a new key pair.
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let (pk, sk) = box_impl::gen_keypair();
    (PublicKey(pk), SecretKey(sk))
}

/// Encrypts some bytes.
pub fn seal(bytes: &[u8], receiver: &PublicKey) -> Vec<u8> {
    sealbox_impl::seal(bytes, &receiver.0)
}

/// Decrypts bytes.
pub fn unseal(bytes: &[u8], secret_key: &SecretKey) -> Option<Vec<u8>> {
    let public_key = secret_key.0.public_key();
    sealbox_impl::open(bytes, &public_key, &secret_key.0).ok()
}
