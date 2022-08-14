# batch_image_invert
Efficient tool to generate inverted images for a folder of originals

Expects a folder (./images/) of original images, for which colours will be inverted, and then saved into the output folder (./inverted/)

For best performance run in release mode by using the --release flag in cargo, for example:
```
cargo run --release
```
