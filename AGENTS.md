# DLT Log Rust Adapter

DLT Log is a Rust crate that provides a [`log`](https://docs.rs/log) adapter for integrating with the Diagnostic Log and Trace (DLT) system. The library generates FFI bindings to the C DLT library using bindgen and implements a logger that sends log messages to the DLT system. This is commonly used in the automotive industry for logging and tracing in embedded systems.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

**CRITICAL**: A cold build (bindgen + dependencies) and coverage runs can take a minute or more. NEVER CANCEL any build, test, or coverage command. Use generous timeouts (120+ seconds).

### Dependencies and Setup
- The devcontainer (`.devcontainer/`) is the reference environment. It installs everything via `.devcontainer/onCreateCommand.sh` and starts the DLT daemon (bound to localhost) on container start via `.devcontainer/postStartCommand.sh`.
- Outside the devcontainer, run `.devcontainer/onCreateCommand.sh` to install dependencies.

### Build Process
- **Build**: `cargo build`
- **Documentation**: `cargo doc`
- **Linting**: `cargo clippy -- -D warnings`
- **Format check**: `cargo fmt -- --check`
- **Pre-commit hooks**: `pre-commit run` (generic file checks plus `cargo fmt` and `cargo clippy`, see `.pre-commit-config.yaml`)

### Testing
**IMPORTANT**: Tests interact with the DLT daemon and require specific setup.

- **Tests without DLT daemon**:
  - Stop daemon: `killall --wait dlt-daemon || true`
  - Run specific test: `cargo test --test init_ok`

- **Tests with DLT daemon**:
  - Start daemon (if not already running, check with `pgrep dlt-daemon`): `./scripts/start-dlt-daemon.sh`
  - Run all tests: `cargo test`
  - `tests/log.rs` uses `dlt-receive` (package `dlt-tools`) to verify that messages reach the daemon.

### Examples and Validation
- **Run example**: `cargo run --example simple`
- **Test with console output**: `DLT_LOCAL_PRINT_MODE=FORCE_ON DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple`
- Expected output shows five log messages with timestamps and DLT log levels `verbose`, `debug`, `info`, `warn`, `error` (mapped from Rust `trace` … `error`).

### Code Coverage
- **Coverage report**: `./scripts/coverage.sh` (uses `cargo llvm-cov`, fails if functions, lines or regions are below 100%)

### Complete CI Validation
- **Full CI**: `./scripts/ci.sh`
- The CI script runs pre-commit, build, doc, clippy, fmt check, tests without and with daemon, and the example in sequence.

## Validation Scenarios

ALWAYS test actual functionality after making changes:

1. **Basic functionality test**:
   - Ensure DLT daemon is running: `./scripts/start-dlt-daemon.sh`
   - Run: `DLT_LOCAL_PRINT_MODE=FORCE_ON DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple`
   - Verify you see all 5 log levels (verbose, debug, info, warn, error) in console output

2. **Integration test**:
   - Run complete test suite: `cargo test`
   - Verify all tests pass

3. **Coverage**:
   - Run `./scripts/coverage.sh` and verify it passes (100% coverage is enforced in CI)

4. **Code quality validation**:
   - Always run `cargo fmt -- --check` before committing
   - Always run `cargo clippy -- -D warnings` before committing
   - CI will fail if these checks don't pass

## Commit Messages

- First line shall have no more than 50 characters and summarize the whole commit.
- Second line is blank.
- All following lines shall have no more than 72 characters.

## Common Tasks

### Repository Structure
```
.
├── README.md             # Main documentation with usage examples (mirrors crate docs in src/lib.rs)
├── CONTRIBUTING.md       # Contributor setup and workflow
├── CHANGELOG.md          # Maintained by release-plz
├── Cargo.toml            # Project configuration, dependencies, metadata
├── build.rs              # Bindgen configuration for FFI generation (allowlist of used DLT functions)
├── src/
│   ├── lib.rs            # Main library code with DLT logger implementation and unit tests
│   ├── libdlt.rs         # Module that includes the generated bindings from OUT_DIR
│   └── libdlt_wrapper.h  # C header wrapper for bindgen
├── examples/
│   └── simple.rs         # Basic usage example showing all log levels
├── tests/                # Integration tests for initialization and logging
├── scripts/
│   ├── ci.sh             # Complete CI validation script
│   ├── coverage.sh       # Coverage report generation and 100% check
│   ├── start-dlt-daemon.sh # Starts dlt-daemon bound to localhost
│   └── set-rust-version.sh # Sets the Rust toolchain in devcontainer.json (used by CI matrix)
├── .github/
│   ├── dependabot.yml
│   └── workflows/
│       ├── ci.yml          # Runs scripts/ci.sh in devcontainer for Rust 1.82, latest, beta, nightly
│       ├── coverage.yml    # Runs scripts/coverage.sh in devcontainer
│       ├── audit.yml       # cargo audit (security advisories)
│       └── release-plz.yml # Release PRs and publishing to crates.io
└── .devcontainer/        # Dev container setup for consistent environment
```

### Key Project Information
- **Language**: Rust (minimum version 1.82, `rust-version` in `Cargo.toml`, tested in CI)
- **Dependencies**: libclang-dev (for bindgen), libdlt-dev (for DLT library), dlt-daemon and dlt-tools (for testing)
- **Build system**: Cargo with custom build.rs for FFI binding generation
- **Testing**: Unit tests + integration tests requiring DLT daemon
- **Coverage requirement**: 100% coverage (enforced in CI)
- **Documentation**: Available at [docs.rs/dlt_log](https://docs.rs/dlt_log)

### Environment-Specific Notes
- The DLT daemon may show "System not booted with systemd!" warnings - this is expected in containerized environments
- FIFO connection errors in logs are normal when daemon runs with restricted permissions
- Cross-compilation requires special setup with BINDGEN_EXTRA_CLANG_ARGS environment variable (see README)

### Build Artifacts
- **Documentation**: Generated in `target/doc/dlt_log/`
- **Coverage report**: `target/coverage/lcov.info` and HTML in `target/coverage/html/`
- **Bindings**: Auto-generated `libdlt_bindings.rs` in build output directory (`OUT_DIR`)

Always build and exercise your changes with the validation scenarios above before considering the task complete.
