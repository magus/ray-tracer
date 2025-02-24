# ray-tracer

```bash
time (rm -f img.ppm && cargo run > img.ppm)

cargo test
```

## TODO

- buffer output to prevent print overhead
- parallelize render, e.g. rayon, calculate multiple pixel rows in parallel?

- viewable render
- incrementally write to output file
- view render as it's happening

- build release cli
- allow arguments for samples per pixel, max depth, dimensions, camera, etc.
