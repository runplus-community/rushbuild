# rushbuild v0.1.0

## Summary

`rushbuild` packages a Rust crate into a self-contained shell runner while
preserving the full source tree inside the bundle.

It is the Rust sibling of [`goshbuild`](https://github.com/runplus-community/goshbuild),
which provides the same model for Go projects.

## Highlights

- single-file delivery for Rust crates
- source-preserving artifact format
- verification before extraction
- build caching per environment
- `rushbuild.sh` and `rushbuild.ps1`
- bundled `demo-app/` for quick validation
- `dist-demo-app/` with generated review outputs

## Tested in this workspace

- `bash -n rushbuild.sh`
- `bash -n test_rushbuild.sh`
- `bash ./test_rushbuild.sh`
- `bash ./dist-demo-app/demo-app.run.sh.test.sh`

## Notes

- The demo runner acceptance suite passed `16/16`.
- First run in a new environment performs a Cargo build, then later runs reuse the cached binary when the cache key matches.
- Generated review outputs can be inspected directly with `bash`.
