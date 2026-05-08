# rushbuild

`rushbuild` is the Rust sibling of [`goshbuild`](https://github.com/runplus-community/goshbuild).

It follows the same source-preserving bundle model for Rust crates:

- keep the full source tree inside the bundle
- ship one runnable `sh` or `ps1` entry point
- verify the payload before extraction
- unpack on first run
- build with `cargo`
- exec the resulting binary with the original arguments

For the Go implementation and reference notes, see [`goshbuild`](https://github.com/runplus-community/goshbuild).

## Status

This repository is the Rust-focused starting point. The goal is to mirror the
same delivery idea from `goshbuild`, but for Rust projects.

## Packaging Flow

```text
+--------------------------+     +-------------------------+     +---------------------------+
| Rust crate source        | --> | rushbuild pack          | --> | generated runner          |
| Cargo.toml / src/*       |     | tar / hash / base64     |     | <name>.run.sh / .ps1      |
+--------------------------+     +-------------------------+     +-------------+-------------+
                                                                                  |
                                                                                  v
                                                     +----------------------------+----------------------------+
                                                     | first run                  | warm cache                 |
                                                     | verify -> extract -> build | cache hit -> exec          |
                                                     +----------------------------+----------------------------+
```

## Notes

- `goshbuild` is the Go reference point.
- `rushbuild` is intended to carry the same delivery pattern into Rust.
- The repo currently starts as a scaffold so the Rust path can be shaped
  before the full packer implementation lands.

