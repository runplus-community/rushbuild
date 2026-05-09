# Changelog

## 0.1.0

Initial Rust implementation of the `rushbuild` source-preserving bundle model.

The project starts with Rust crates and keeps room for future language lanes such
as Go, Python, and other automation stacks. [`goshbuild`](https://github.com/runplus-community/goshbuild)
is the Go-focused reference implementation.

### Added

- `rushbuild.sh` for Rust crates
- `rushbuild.ps1` as the Windows PowerShell entry wrapper
- source-preserving packaging of Cargo projects into a single runnable `.sh`
- documented the broader multi-language direction while keeping Rust as the current implementation
- payload verification before extraction
- per-environment build cache keyed by package, Rust host, rustc version, and payload hash
- generated acceptance tests for the packaged runner
- bundled `demo-apps/rust-demo/` Rust crate
- `dists/dist-rust-demo/` review outputs
- reserved `demo-apps/golang-demo/` and `dists/dist-golang-demo/` for future Go support
- GitHub Actions CI workflow

### Validation

- `bash -n rushbuild.sh`
- `bash -n test_rushbuild.sh`
- `cargo generate-lockfile` in `demo-apps/rust-demo/`
- `bash ./test_rushbuild.sh`
- `bash ./dists/dist-rust-demo/demo-app.run.sh.test.sh` passed `16/16`
