# Build

## Windows

```cargo build --release --target x86_64-pc-windows-msvc```

## PGO

Build with PGO tools

```RUSTFLAGS="-C profile-generate=/tmp/pgo" cargo build --release```

Run app

```./target/release/my_app --load-data=test.json --serve```

Build using PGO profiling

```RUSTFLAGS="-C profile-use=/tmp/pgo -C llvm-profile-cleanup=true" cargo build --release```
