# ray-tracer

```bash
# debug
time (rm -f img.ppm && cargo run > img.ppm)

# release
cargo build --release
time (rm -f img.ppm && ./target/release/ray-tracer > img.ppm)

# test
cargo test
```

## TODO

- viewable render
- incrementally write to output file
- view render as it's happening

- build release cli
- allow arguments for samples per pixel, max depth, dimensions, camera, etc.
