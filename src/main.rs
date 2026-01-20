use bip32::{Prefix, XPrv};
use clap::Parser;
use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroize;

/// Simple BIP39 + BIP32 seed generator
#[derive(Parser, Debug)]
#[command(name = "seedgen", about = "Generate a mnemonic and BIP32 seed")]
struct Args {
    /// Print raw hex BIP32 seed instead of mnemonic/xprv/xpub
    #[arg(long)]
    seed_only: bool,
}

fn main() {
    let args = Args::parse();

    // 256-bit entropy
    let mut entropy = [0u8; 32];
    OsRng.fill_bytes(&mut entropy);

    // Build 24-word English BIP39 mnemonic from entropy
    let mnemonic = bip39::Mnemonic::from_entropy_in(bip39::Language::English, &entropy)
        .expect("failed to create mnemonic from entropy");

    // Convert to 64-byte BIP32 seed (PBKDF2 w/ 2048 rounds).
    // Empty passphrase by default.
    let mut seed = mnemonic.to_seed_normalized("");

    // Derive master keys
    let xprv = XPrv::new(seed).expect("xprv derivation failed");
    let xpub = xprv.public_key();

    // output results
    if args.seed_only {
        println!("{}", hex::encode(seed));
    } else {
        println!("Mnemonic : {mnemonic}");
        println!("BIP32 Seed (hex) : {}", hex::encode(seed));

        // Displayed using Bitcoin mainnet prefixes
        let xprv_str = xprv.to_string(Prefix::XPRV); // Zeroizing<String>
        let xpub_str = xpub.to_string(Prefix::XPUB); // Zeroizing<String>
        println!("Master XPRV      : {}", xprv_str.as_str());
        println!("Master XPUB      : {}", xpub_str.as_str());
    }

    // Zero sensitive buffers
    entropy.zeroize();
    seed.zeroize();
    // xprv/xpub strings are zeroized on drop by the Zeroizing wrapper
}
