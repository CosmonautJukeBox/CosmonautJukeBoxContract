# Cosmonaut Juke Box Contract

This contract allows to select a color and a song to play on the Cosmonaut Juke Box.

The color is stored as merkle tree in the contract state.

The contract can be deployed to the Neutron Test Net.

## Schema

To generate the schema, run the following command:

```bash
cargo schema
```

## Build

Build the contract with the following command:

```bash
cargo wasm
```

## Test

Run the tests with the following command:

```bash
cargo unit-test
```

## Optimize

Optimize the contract with the following command:

```bash
cargo install cargo-run-script
cargo run-script optimize
```

## Deploy

Copy the example.env to .env and fill in the MNEMONIC.

Deploy the contract with the following command:

```bash
RUST_LOG=info cargo run --bin deploy
```
