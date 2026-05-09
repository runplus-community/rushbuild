# Changelog

## 0.1.0

Initial Rust mirror of [`goshbuild`](https://github.com/runplus-community/goshbuild).

### Added

- `rushbuild.sh` for Rust crates
- `rushbuild.ps1` as the Windows PowerShell entry wrapper
- source-preserving packaging of Cargo projects into a single runnable `.sh`
- payload verification before extraction
- per-environment build cache keyed by package, Rust host, rustc version, and payload hash
- generated acceptance tests for the packaged runner
- bundled `demo-app/` Rust crate
- `dist-demo-app/` review outputs
- GitHub Actions CI workflow

### Validation

- `bash -n rushbuild.sh`
- `bash -n test_rushbuild.sh`
- `cargo generate-lockfile` in `demo-app/`
- `bash ./test_rushbuild.sh`
- `bash ./dist-demo-app/demo-app.run.sh.test.sh` passed `16/16`

