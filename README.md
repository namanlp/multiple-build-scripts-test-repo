This is repository for multiple build script feature for my GSoC contribution at The Rust Foundation.

There are primarily 2 build scripts in this, and they have tests for various things like setting environment variables, building C libraries etc.

Running with nightly

```
rustup run nightly cargo run
```

Running with custom cargo build

```
rustup run nightly <path>/cargo/target/release/cargo run
```
