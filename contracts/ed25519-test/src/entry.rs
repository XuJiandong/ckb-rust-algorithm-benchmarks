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
use ed25519_dalek::{self, Signature, SigningKey, VerifyingKey};
use hex_literal::hex;
use sha2::{digest::Digest, Sha512};

pub fn main() -> Result<(), Error> {
    // test case from: https://github.com/dalek-cryptography/ed25519-dalek/blob/02001d8c3422fb0314b541fdb09d04760f7ab4ba/tests/ed25519.rs#L102

    let sec_bytes = hex!("833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42");
    let pub_bytes = hex!("ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf");
    let msg_bytes = hex!("616263");
    let sig_bytes = hex!("98a70222f0b8121aa9d30f813d683f809e462b469c7ff87639499bb94e6dae4131f85042463c2a355a2003d062adf5aaa10b8c61e636062aaad11c2a26083406");

    let signing_key = SigningKey::from_bytes(&sec_bytes);
    let expected_verifying_key = VerifyingKey::from_bytes(&pub_bytes).unwrap();
    assert_eq!(expected_verifying_key, signing_key.verifying_key());
    let sig1 = Signature::try_from(&sig_bytes[..]).unwrap();

    let mut prehash_for_signing = Sha512::default();
    let mut prehash_for_verifying = Sha512::default();

    prehash_for_signing.update(&msg_bytes[..]);
    prehash_for_verifying.update(&msg_bytes[..]);

    let sig2: Signature = signing_key
        .sign_prehashed(prehash_for_signing, None)
        .unwrap();

    assert!(
        sig1 == sig2,
        "Original signature from test vectors doesn't equal signature produced:\
         \noriginal:\n{:?}\nproduced:\n{:?}",
        sig1,
        sig2
    );
    let start_cycles = current_cycles();
    assert!(
        signing_key
            .verify_prehashed(prehash_for_verifying.clone(), None, &sig2)
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
