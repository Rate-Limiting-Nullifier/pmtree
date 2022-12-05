<p align="center">
    <img src="./pmtree.png" width="150">
</p>

<p align="center">
    <img src="https://github.com/Rate-Limiting-Nullifier/pmtree/workflows/Build-Test-Fmt/badge.svg" width="140">
</p>

<h1 align="center">pmtree</h1>

<h3 align="center">Persistent Merkle Tree (optimized & sparse & fixed-size) in Rust</h3>

## How to use
```toml
[dependencies]
pmtree = { git = "https://github.com/Rate-Limiting-Nullifier/pmtree" }
```

To use batch insertions you must enable `batch_insert` feature:
```toml
[dependencies]
pmtree = { git = "https://github.com/Rate-Limiting-Nullifier/pmtree", features = ["batch_insert"] }
```

## Clone & Build
```bash
git clone git@github.com:Rate-Limiting-Nullifier/pmtree.git && cd pmtree
cargo build --release
```

## Run tests
```bash
cargo test --release
```

## Docs
```bash
cargo docs --open
```
