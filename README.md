# seedgen

A small CLI for generating **BIP39 mnemonics** and **BIP32 seeds**.
Useful for setting up hot wallets or providing a `BIP32_SEED` to other services.

## Installation

### Build from source
Requires Rust **1.88+**.

```bash
git clone https://github.com/sovafoundation/seedgen
cd seedgen
cargo build --release
./target/release/seedgen
```

### Run with Docker
Images are published to GitHub Container Registry:

```bash
docker run --rm ghcr.io/sovafoundation/seedgen:latest
```

To output only the raw hex seed (e.g. for `BIP32_SEED`):

```bash
docker run --rm ghcr.io/sovafoundation/seedgen:latest --seed-only
```

## Usage

Generate mnemonic + seed + xprv/xpub:
```bash
./seedgen
```

Generate seed only:
```bash
./seedgen --seed-only
```

## Security

This tool uses the zeroize crate to wipe sensitive data (entropy, seeds, private keys) from memory after each run.
Without zeroization, those secrets could remain in memory until overwritten, potentially exposing them via crash dumps or memory leaks.

## License

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
