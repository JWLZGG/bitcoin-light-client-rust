# bitcoin-light-client-rust
<img width="201" height="201" alt="Starkle" src="https://github.com/user-attachments/assets/6c00a94b-d0ee-449a-af02-93a7f7572f5b" />


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
