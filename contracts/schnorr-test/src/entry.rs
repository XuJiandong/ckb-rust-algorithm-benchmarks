#![allow(dead_code)]
#![allow(unused_imports)]
use core::result::Result;
use alloc::format;
use alloc::vec::Vec;

use ckb_std::syscalls::debug;
use k256::schnorr::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
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
fn gen() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let sk = SigningKey::from_bytes(&SECRET_KEY).unwrap();
    let pk = sk.verifying_key();
    let msg_bytes = generate_msg();
    let signature = sk.try_sign(&msg_bytes).unwrap();

    debug(format!("msg_bytes = {}", hex::encode(&msg_bytes)));

    let pub_bytes = pk.to_bytes();
    debug(format!("pub_bytes = {}", hex::encode(&pub_bytes)));
    let sig_bytes = signature.to_bytes();
    debug(format!("sig_bytes = {}", hex::encode(&sig_bytes)));

    (msg_bytes.to_vec(), pub_bytes.to_vec(), sig_bytes.to_vec())
}

#[cfg(not(feature = "include_signing"))]
fn gen() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    use alloc::vec::Vec;

    let msg_bytes =
        hex::decode("68656c6c6f2c20776f726c640000000000000000000000000000000000000000").unwrap();
    let pub_bytes =
        hex::decode("0cec028ee08d09e02672a68310814354f9eabfff0de6dacc1cd3a774496076ae").unwrap();
    let sig_bytes = hex::decode("e3ad8bcbe047e2bbaaf81b653c49545533f739c035356f4e3a97ccccb0b4e69649b70ce7b45a20308bab1cff4c44295504894de146c19287b917c7506506ffc9").unwrap();
    (msg_bytes, pub_bytes, sig_bytes)
}

pub fn main() -> Result<(), Error> {
    let (msg_bytes, pub_bytes, sig_bytes) = gen();
    let signature = Signature::try_from(sig_bytes.as_slice()).unwrap();
    let pk = VerifyingKey::from_bytes(&pub_bytes).unwrap();

    let last = current_cycles();
    pk.verify(&msg_bytes, &signature).unwrap();
    let cycles = current_cycles() - last;
    debug(format!(
        "cost of schnorr verifying cycles: {} K",
        cycles / 1024
    ));

    Ok(())
}
