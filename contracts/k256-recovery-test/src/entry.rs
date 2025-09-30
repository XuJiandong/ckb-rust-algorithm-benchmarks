#![allow(dead_code)]
#![allow(unused_imports)]

// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::format;
use alloc::vec::Vec;

use ckb_std::syscalls::debug;
use k256::ecdsa::{
    signature::{Signer, Verifier},
    RecoveryId, Signature, SigningKey, VerifyingKey,
};
use k256::sha2::{Digest, Sha256};
// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use crate::error::Error;
use ckb_std::syscalls::current_cycles;

const SECRET_KEY: [u8; 32] = [
    0x51, 0x9b, 0x42, 0x3d, 0x71, 0x5f, 0x8b, 0x58, 0x1f, 0x4f, 0xa8, 0xee, 0x59, 0xf4, 0x77, 0x1a,
    0x5b, 0x44, 0xc8, 0x13, 0x0b, 0x4e, 0x3e, 0xac, 0xca, 0x54, 0xa5, 0x6d, 0xda, 0x72, 0xb4, 0x64,
];

fn generate_msg() -> [u8; 32] {
    let hello = b"hello, world";
    let mut msg = [0u8; 32];
    msg[0..hello.len()].copy_from_slice(hello);
    msg
}

#[cfg(feature = "include_signing")]
fn gen() -> (Vec<u8>, Vec<u8>, Vec<u8>, u8) {
    let sk = SigningKey::from_slice(&SECRET_KEY).unwrap();
    let pk = sk.verifying_key();
    let msg_bytes = generate_msg();

    let (signature, rec_id) = sk.sign_prehash_recoverable(&msg_bytes).unwrap();

    debug(format!("msg_bytes = {}", hex::encode(&msg_bytes)));

    let pub_bytes = pk.to_sec1_bytes();
    debug(format!("pub_bytes = {}", hex::encode(&pub_bytes)));

    let sig_bytes = signature.to_vec();
    debug(format!("sig_bytes = {}", hex::encode(&sig_bytes)));
    debug(format!("recId = {}", rec_id.to_byte()));

    (
        msg_bytes.to_vec(),
        pub_bytes.to_vec(),
        sig_bytes.to_vec(),
        rec_id.to_byte(),
    )
}

#[cfg(not(feature = "include_signing"))]
fn gen() -> (Vec<u8>, Vec<u8>, Vec<u8>, u8) {
    use alloc::vec::Vec;

    let msg_bytes =
        hex::decode("68656c6c6f2c20776f726c640000000000000000000000000000000000000000").unwrap();
    let pub_bytes =
        hex::decode("030cec028ee08d09e02672a68310814354f9eabfff0de6dacc1cd3a774496076ae").unwrap();
    let sig_bytes = hex::decode("bb54baa41daefb2b6931ddbbbb636054b9027e966a61c3bf5fb75e26b2f2a78f1c65827dba40d81242ae3362e3a936e18092f21d5be8a1ff1b7693cc85068d3a").unwrap();
    let rec_id = 1u8;
    (msg_bytes, pub_bytes, sig_bytes, rec_id)
}

pub fn main() -> Result<(), Error> {
    let (msg_bytes, pub_bytes, sig_bytes, rec_id) = gen();
    let signature = Signature::from_slice(&sig_bytes).unwrap();
    let pk = VerifyingKey::from_sec1_bytes(&pub_bytes).unwrap();
    let rec_id = RecoveryId::try_from(rec_id).unwrap();

    let last = current_cycles();
    let recovered_key =
        VerifyingKey::recover_from_prehash_noverify(&msg_bytes, &signature, rec_id).unwrap();
    assert_eq!(recovered_key, pk);
    let _recovered_key_bytes = recovered_key.to_sec1_bytes();
    let cycles = current_cycles() - last;
    debug(format!(
        "cost of k256(recovery) verifying cycles: {} K",
        cycles / 1024
    ));

    Ok(())
}
