# rushbuild v0.1.0

## Summary

`rushbuild` packages source projects into self-contained shell runners while
preserving the full source tree inside the bundle.

The current implementation supports Rust crates. It starts as the Rust mirror of
[`goshbuild`](https://github.com/runplus-community/goshbuild), while leaving room
for future language lanes such as Go, Python, and other automation stacks.

## Highlights

- single-file delivery for Rust crates
- source-preserving artifact format
- verification before extraction
- build caching per environment
- documented multi-language direction with Rust as the first supported lane
- `rushbuild.sh` and `rushbuild.ps1`
- bundled `demo-apps/rust-demo/` for quick validation
- `dists/dist-rust-demo/` with generated review outputs
- reserved Go demo and dist folders for future language support

## Tested in this workspace

- `bash -n rushbuild.sh`
- `bash -n test_rushbuild.sh`
- `bash ./test_rushbuild.sh`
- `bash ./dists/dist-rust-demo/demo-app.run.sh.test.sh`

## Notes

- The demo runner acceptance suite passed `16/16`.
- First run in a new environment performs a Cargo build, then later runs reuse the cached binary when the cache key matches.
- Generated review outputs can be inspected directly with `bash`.
