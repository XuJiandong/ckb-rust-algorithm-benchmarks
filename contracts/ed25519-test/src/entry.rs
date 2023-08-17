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
use ed25519_dalek::{self, Signature, VerifyingKey};
use hex_literal::hex;
use sha2::{digest::Digest, Sha512};

pub fn main() -> Result<(), Error> {
    // test case from: https://github.com/dalek-cryptography/ed25519-dalek/blob/02001d8c3422fb0314b541fdb09d04760f7ab4ba/tests/ed25519.rs#L102
    let pub_bytes = hex!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf");
    let msg_bytes = hex!("616263");
    let sig_bytes = hex!("98a70222f0b8121aa9d30f813d683f809e462b469c7ff87639499bb94e6dae4131f85042463c2a355a2003d062adf5aaa10b8c61e636062aaad11c2a26083406");

    let expected_verifying_key = VerifyingKey::from_bytes(&pub_bytes).unwrap();
    let sig1 = Signature::try_from(&sig_bytes[..]).unwrap();

    let mut prehash_for_verifying = Sha512::default();
    prehash_for_verifying.update(&msg_bytes[..]);

    let start_cycles = current_cycles();
    assert!(
        expected_verifying_key
            .verify_prehashed(prehash_for_verifying.clone(), None, &sig1)
            .is_ok(),
        "Could not verify ed25519ph signature!"
    );
    let cycles = current_cycles() - start_cycles;
    debug(format!(
        "cost of ed25519 verifying cycles: {} K",
        cycles / 1024
    ));

    Ok(())
}
