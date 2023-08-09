// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::format;

use ckb_std::syscalls::debug;
use k256::ecdsa::{
    signature::{Signer, Verifier},
    SigningKey,
};
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

fn k256() -> Result<(), Error> {
    let sk = SigningKey::from_slice(&SECRET_KEY).unwrap();
    let pk = sk.verifying_key();
    let msg = generate_msg();
    let (signature, _) = sk.try_sign(&msg).unwrap();
    let last = current_cycles();
    pk.verify(&msg, &signature).unwrap();
    let cycles = current_cycles() - last;
    debug(format!(
        "cost of k256 verifying cycles: {} K",
        cycles / 1024
    ));

    Ok(())
}

pub fn main() -> Result<(), Error> {
    k256()?;
    Ok(())
}
