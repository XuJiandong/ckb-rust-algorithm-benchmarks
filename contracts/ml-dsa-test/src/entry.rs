use alloc::vec::Vec;
use core::result::Result;

use crate::error::Error;
use alloc::format;
use ckb_std::syscalls::{current_cycles, debug};
use ml_dsa::{EncodedVerifyingKey, MlDsa44, MlDsa65, MlDsa87, Signature, VerifyingKey};

pub fn main() -> Result<(), Error> {
    let args = ckb_std::env::argv();
    let type_bytes = args[0].to_bytes();
    let vk_bytes: Vec<u8> = hex::decode(args[1].to_bytes()).unwrap();
    let msg_bytes: Vec<u8> = hex::decode(args[2].to_bytes()).unwrap();
    let sig_bytes: Vec<u8> = hex::decode(args[3].to_bytes()).unwrap();
    let ctx = [0u8; 8];

    let last = current_cycles();

    match type_bytes {
        b"MlDsa44" => {
            let enc_vk = EncodedVerifyingKey::<MlDsa44>::try_from(vk_bytes.as_slice()).unwrap();
            let vk = VerifyingKey::<MlDsa44>::decode(&enc_vk);
            let sig = Signature::<MlDsa44>::try_from(sig_bytes.as_slice()).unwrap();
            assert!(vk.verify_with_context(&msg_bytes, &ctx, &sig));
        }
        b"MlDsa65" => {
            let enc_vk = EncodedVerifyingKey::<MlDsa65>::try_from(vk_bytes.as_slice()).unwrap();
            let vk = VerifyingKey::<MlDsa65>::decode(&enc_vk);
            let sig = Signature::<MlDsa65>::try_from(sig_bytes.as_slice()).unwrap();
            assert!(vk.verify_with_context(&msg_bytes, &ctx, &sig));
        }
        b"MlDsa87" => {
            let enc_vk = EncodedVerifyingKey::<MlDsa87>::try_from(vk_bytes.as_slice()).unwrap();
            let vk = VerifyingKey::<MlDsa87>::decode(&enc_vk);
            let sig = Signature::<MlDsa87>::try_from(sig_bytes.as_slice()).unwrap();
            assert!(vk.verify_with_context(&msg_bytes, &ctx, &sig));
        }
        _ => panic!("Unknown type"),
    }

    let cycles = current_cycles() - last;
    debug(format!(
        "cost of ml-dsa ({}) verifying cycles: {} K",
        core::str::from_utf8(type_bytes).unwrap_or("?"),
        cycles / 1024
    ));

    Ok(())
}
