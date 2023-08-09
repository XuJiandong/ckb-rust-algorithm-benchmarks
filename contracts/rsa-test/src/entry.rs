// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::format;

use ckb_std::syscalls::debug;
use rsa::{
    pkcs1v15,
    pkcs8::DecodePrivateKey,
    signature::{Keypair, SignatureEncoding, Signer, Verifier},
    RsaPrivateKey,
};
use rsa::sha2::Sha256;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use crate::error::Error;
use ckb_std::syscalls::current_cycles;

fn generate_msg() -> [u8; 32] {
    let hello = b"hello, world";
    let mut msg = [0u8; 32];
    msg[0..hello.len()].copy_from_slice(hello);
    msg
}

pub fn test_rsa_2048(msg: &[u8]) {
    let pem = include_str!("rsa2048-priv.pem");
    let private_key = RsaPrivateKey::from_pkcs8_pem(pem).unwrap();

    let signing_key = pkcs1v15::SigningKey::<Sha256>::new(private_key);
    let signature_bytes = signing_key.sign(msg).to_bytes();

    let verifying_key = signing_key.verifying_key();
    let signature = pkcs1v15::Signature::try_from(&*signature_bytes).unwrap();

    let last = current_cycles();
    verifying_key.verify(&msg, &signature).unwrap();
    let cycles = current_cycles() - last;
    debug(format!("cost of verifying cycles: {} K", cycles / 1024));
}

pub fn main() -> Result<(), Error> {
    let msg = generate_msg();
    test_rsa_2048(&msg);

    Ok(())
}

// Unit tests are supported.
#[test]
fn test_foo() {
    assert!(true);
}
