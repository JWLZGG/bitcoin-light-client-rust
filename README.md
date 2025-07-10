# bitcoin-light-client-rust

A minimal Bitcoin light client written in Rust that:
- Syncs and validates block headers
- Verifies SPV proofs
- Tracks UTXOs
- (Optionally) Exports zk-verifiable traces

## Usage

```bash
cargo run -- generate-address
cargo run -- fetch-utxos <address>
```