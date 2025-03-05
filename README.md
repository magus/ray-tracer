# ray-tracer

```bash
# debug
time cargo run

# release
cargo build --release
time ./target/release/ray-tracer

# one command release build render
cargo build --release && time ./target/release/ray-tracer

# test
cargo test
```

## TODO

- render visualization
- send pixel+index updates every 16ms (60fps)
- bidirectional websocket?
- webapp could work but consider alternate approaches like custom gui

- build release cli
- allow arguments for samples per pixel, max depth, dimensions, camera, etc.
- keyboard / cli control to adjust camera, edit items in scene, add/remove items to scene, etc.
