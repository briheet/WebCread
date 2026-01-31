# WebCread

A rust implementation for webcam reading. Previous was written via IOCTL and Linux subsystem.
Hobby project, just to fck with claude review and trying to rewrite something.

## Usage

**With Cargo:**
```bash
# This is slow due to not optimized build. Use cargo build --release for better viewing.
cargo run

# Use this for better viewing
cargo build --release && ./target/release/WebCread
```

**With Nix:**
```bash
nix run github:briheet/WebCread
```
