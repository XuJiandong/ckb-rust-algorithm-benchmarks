// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::format;

use ckb_std::syscalls::debug;
// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use crate::error::Error;
use ckb_std::syscalls::current_cycles;
use p256::ecdsa::{
    signature::hazmat::{PrehashSigner, PrehashVerifier},
    SigningKey,
};

pub fn main() -> Result<(), Error> {
    let sk = SigningKey::from_slice([1u8; 32].as_ref()).unwrap();
    let pk = sk.verifying_key();
    let msg = [0u8; 32];
    let (signature, _) = sk.sign_prehash(&msg).unwrap();
    let last = current_cycles();
    pk.verify_prehash(&msg, &signature).unwrap();
    let cycles = current_cycles() - last;
    debug(format!("cost of verifying cycles: {} K", cycles / 1024));

    Ok(())
}

// Unit tests are supported.
#[test]
fn test_foo() {
    assert!(true);
}
