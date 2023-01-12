Utility tool that creates an image containing all printable ASCII characters rendered with a given font and font size with constant spacing, passed as command line arguments using the clap library. It does panic on error.

Build and example usage using Roboto Mono:

```
cargo build --release
cargo run -- -s 24 -f roboto_mono.ttf -o output.png
```

produces image that looks like this (cropped):

img link