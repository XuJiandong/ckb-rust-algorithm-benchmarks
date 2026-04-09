use ml_dsa::{KeyGen, MlDsa44, MlDsa65, MlDsa87, Seed};
use ml_dsa::signature::Keypair;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let type_arg = args
        .get(1)
        .expect("Usage: ml-dsa-signing-tool <MlDsa44|MlDsa65|MlDsa87>");

    let seed = Seed::default();
    let msg = b"Hello world";
    let ctx = [0u8; 8];

    match type_arg.as_str() {
        "MlDsa44" => {
            let ssk = MlDsa44::from_seed(&seed);
            let sk = ssk.signing_key();
            let vk = ssk.verifying_key();
            let sig = sk.sign_deterministic(msg, &ctx).unwrap();
            println!(
                "MlDsa44 {} {} {}",
                hex::encode(vk.encode()),
                hex::encode(msg),
                hex::encode(sig.encode()),
            );
        }
        "MlDsa65" => {
            let ssk = MlDsa65::from_seed(&seed);
            let sk = ssk.signing_key();
            let vk = ssk.verifying_key();
            let sig = sk.sign_deterministic(msg, &ctx).unwrap();
            println!(
                "MlDsa65 {} {} {}",
                hex::encode(vk.encode()),
                hex::encode(msg),
                hex::encode(sig.encode()),
            );
        }
        "MlDsa87" => {
            let ssk = MlDsa87::from_seed(&seed);
            let sk = ssk.signing_key();
            let vk = ssk.verifying_key();
            let sig = sk.sign_deterministic(msg, &ctx).unwrap();
            println!(
                "MlDsa87 {} {} {}",
                hex::encode(vk.encode()),
                hex::encode(msg),
                hex::encode(sig.encode()),
            );
        }
        _ => panic!("Unknown type: {type_arg}. Expected MlDsa44, MlDsa65, or MlDsa87."),
    }
}
