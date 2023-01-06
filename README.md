## File Server using Rocket(Rust)

### Features
1. Upload File to IPFS using NFT Storage API

# Build server
```bash
cargo build --release
```

# Run Server
```bash
./target/release/dot_marketplace_file_server
```
or 
```bash
cargo  run
```

# Tests
Note: Run the tests using a single thread
```bash
cargo test -- --test-threads=1
```
