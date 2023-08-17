#![allow(dead_code)]
#![allow(unused_imports)]

// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{format, vec::Vec};

use ckb_std::syscalls::debug;
// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use crate::error::Error;
use ckb_std::syscalls::current_cycles;
use hex;
use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};

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
fn gen() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sk = SigningKey::from_slice(&SECRET_KEY).unwrap();
    let pk = sk.verifying_key();
    let msg_bytes = generate_msg();
    let (signature, _) = sk.try_sign(&msg_bytes).unwrap();

    debug(format!("msg_bytes = {}", hex::encode(&msg_bytes)));

    let pub_bytes = pk.to_sec1_bytes();
    debug(format!("pub_bytes = {}", hex::encode(&pub_bytes)));
    let sig_bytes = signature.to_vec();
    debug(format!("sig_bytes = {}", hex::encode(&sig_bytes)));

    (msg_bytes.to_vec(), pub_bytes.to_vec(), sig_bytes.to_vec())
}

#[cfg(not(feature = "include_signing"))]
fn gen() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let msg_bytes =
        hex::decode("68656c6c6f2c20776f726c640000000000000000000000000000000000000000").unwrap();
    let pub_bytes = hex::decode("041ccbe91c075fc7f4f033bfa248db8fccd3565de94bbfb12f3c59ff46c271bf83ce4014c68811f9a21a1fdb2c0e6113e06db7ca93b7404e78dc7ccd5ca89a4ca9").unwrap();
    let sig_bytes = hex::decode("ec526407bff890bc53fdef6c1d9951c5a7161f7707ac4cf526f768331b02c68350ceddffc143f53975bc104d04a79d25caf1e5afbbf5f7ec2ad4a214af9fa1c8").unwrap();
    (msg_bytes, pub_bytes, sig_bytes)
}

pub fn main() -> Result<(), Error> {
    let (msg_bytes, pub_bytes, sig_bytes) = gen();
    let signature = Signature::from_slice(&sig_bytes).unwrap();
    let pk = VerifyingKey::from_sec1_bytes(&pub_bytes).unwrap();

    let last = current_cycles();
    pk.verify(&msg_bytes, &signature).unwrap();
    let cycles = current_cycles() - last;
    debug(format!(
        "cost of p256 verifying cycles: {} K",
        cycles / 1024
    ));

    Ok(())
}
