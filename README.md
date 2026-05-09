# rushbuild

`rushbuild` packages a Rust crate into a self-contained shell runner.

The bundle preserves the crate source and build inputs, then emits a single
`sh` entry point that extracts, verifies, builds, and executes the Rust program
at runtime.

`rushbuild` is the Rust mirror of [`goshbuild`](https://github.com/runplus-community/goshbuild).
Use `goshbuild` for Go projects and `rushbuild` for Rust projects.

Root-level entry points:

- `rushbuild.sh` for shell environments
- `rushbuild.ps1` as the PowerShell wrapper for Windows
- `test_rushbuild.sh` as the higher-order demo harness

## Quick Start

```bash
bash ./test_rushbuild.sh
bash ./dist-demo-app/demo-app.run.sh --help
```

```powershell
bash ./test_rushbuild.sh
bash .\dist-demo-app\demo-app.run.sh --help
```

## Output

`pack` creates:

- `<name>.run.sh` - the self-contained runner
- `<name>.run.sh.test.sh` - acceptance tests for the runner

The generated runner embeds a tarball of the crate source, verifies the
payload hash, extracts into a cache directory, builds the Rust binary with
`cargo build --release`, and then `exec`s the binary with the original
arguments.

In the demo flow, the generated scripts land in `dist-demo-app/`,
and you can run the `.run.sh` and `.test.sh` files directly with `bash`.

## Packing Flow

```text
+-------------------------------+     +-------------------------------+     +-------------------------------+
| RUST CRATE SOURCE             | --> | RUSHBUILD PACK                | --> | GENERATED RUNNER              |
| Cargo.toml / Cargo.lock / src |     | vendor / tar / hash / b64     |     | <name>.run.sh                 |
| source stays in payload       |     | runner stub + checksum        |     | one runnable file             |
+-------------------------------+     +-------------------------------+     +---------------+---------------+
                                                                                        |
                                                                                        v
                                                           +----------------------------+----------------------------+
                                                           | COLD CACHE                 | WARM CACHE                 |
                                                           | first run in new env       | later runs in same env     |
                                                           | verify -> extract -> build | cache hit -> exec          |
                                                           | exec                       |                            |
                                                           +----------------------------+----------------------------+
```

## Behavior

- Any Rust crate can be delivered as a single `sh` entry point.
- The source tree remains multi-file and inspectable, while the handoff artifact stays single-file.
- The first run in a new environment performs a Cargo build; later runs reuse the cached binary when the cache key matches.
- Payload verification happens before extraction, which provides a corruption check before any source is unpacked.
- Cache keys include package identity, Rust host triple, rustc version, and payload hash.
- `cargo vendor` runs before packing so vendored dependencies can travel with the payload.
- `goshbuild` provides the same model for Go modules.

## Use Cases

### 1. CI/CD helper

A repository needs a Rust-based helper, release step, or validation shim that
should be delivered as one runnable file. `rushbuild` packages that helper into
one runner while preserving the source.

### 2. Repeated internal automation

Teams can package Rust tools for code generation, validation, repo maintenance,
or release automation. The first run builds the binary; repeat runs hit the
cache until the source, toolchain, or target host changes.

### 3. Support and incident-response bundle

An ops or support team can package a Rust recovery tool and hand off one
`.run.sh` file. The runner verifies the payload before extraction and builds a
fresh binary in the target environment.

### 4. Release handoff

The source project can remain split across many files, modules, and Cargo
metadata, while the delivered artifact stays one runnable file.

## Demo app

`demo-app/` contains only the Rust crate. The higher-order bundle script is at
[`test_rushbuild.sh`](test_rushbuild.sh) in the repo root and packs the demo app
into `dist-demo-app/`.

```bash
bash ./test_rushbuild.sh
bash ./dist-demo-app/demo-app.run.sh --help
bash ./dist-demo-app/demo-app.run.sh.test.sh
```

```powershell
bash ./test_rushbuild.sh
bash .\dist-demo-app\demo-app.run.sh --help
bash .\dist-demo-app\demo-app.run.sh.test.sh
```

## Post-build Outputs

```text
dist-demo-app/
|-- demo-app.run.sh
|-- demo-app.run.sh.test.sh
`-- demo-app.run.corrupt.sh
```

You can run these scripts directly with `bash`.

For a runner-focused breakdown, see [dist-demo-app/README.md](dist-demo-app/README.md).

## Validation

Current workspace validation:

- `bash -n ./rushbuild.sh`
- `bash -n ./test_rushbuild.sh`
- PowerShell parse of `rushbuild.ps1`
- `bash ./test_rushbuild.sh`
- `bash ./dist-demo-app/demo-app.run.sh.test.sh`
- GitHub Actions CI workflow added at [.github/workflows/ci.yml](.github/workflows/ci.yml)

The demo test suite passed `16/16` in this workspace.

## Requirements

- `cargo`
- `rustc`
- `tar`
- `base64`
- `bash` for the generated runner

## Go Version

For Go projects, use [`goshbuild`](https://github.com/runplus-community/goshbuild).

## Release Notes

- [CHANGELOG.md](CHANGELOG.md)
- [RELEASE.md](RELEASE.md)

## License

MIT. See [LICENSE](LICENSE).
