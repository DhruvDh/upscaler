# Upscaler

Upscaling images for fun and profit.

## Installation

First ensure, `rustup` is installed. If not, got to https://rustup.rs/ and follow the instructions.

For Unix environments, install `rustup` by running -

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, after restarting shell, run -

```
cargo install --git https://github.com/DhruvDh/upscaler
```

## Usage

See via `upscaler --help`

```
Upscaler 0.1.0
Dhruv D. <ddhamani@uncc.edu>
Upscales images

USAGE:
    upscaler [OPTIONS] <INPUT> <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --scale <scale>    Scaling factor [default: 2]  [possible values: 2, 4, 8, 16, 32]

ARGS:
    <INPUT>     The input image to upscale [possible types: hdr, bmp tga, tiff, dxt, gif, jpeg, png, pnm, webp]
    <OUTPUT>    The output image to write [possible types: jpeg, png]
```