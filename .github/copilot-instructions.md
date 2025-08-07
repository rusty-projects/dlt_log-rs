# DLT Log Rust Adapter

DLT Log is a Rust crate that provides a [`log`](https://docs.rs/log) adapter for integrating with the Diagnostic Log and Trace (DLT) system. The library generates FFI bindings to the C DLT library using bindgen and implements a logger that sends log messages to the DLT system. This is commonly used in the automotive industry for logging and tracing in embedded systems.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

**CRITICAL**: ALL build and test commands take significant time. NEVER CANCEL any build, test, or coverage command. Set explicit timeouts as shown below.

### Dependencies and Setup
- Install system dependencies: `sudo apt-get update && sudo apt-get install -y libclang-dev libdlt-dev dlt-daemon`
- Minimum Rust version: 1.82 (due to bindgen requirements)
- Install coverage tools: `rustup component add llvm-tools-preview`
- Install cargo-llvm-cov: `curl -LsSf https://github.com/taiki-e/cargo-llvm-cov/releases/download/v0.6.16/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz | tar xzf - -C ~/.cargo/bin`

### Build Process
- **Build**: `cargo build` -- takes 40 seconds on first build. NEVER CANCEL. Set timeout to 120+ seconds.
- **Documentation**: `cargo doc` -- takes 3 seconds. Set timeout to 30+ seconds.
- **Linting**: `cargo clippy -- -D warnings` -- takes 2 seconds. Set timeout to 30+ seconds.
- **Format check**: `cargo fmt -- --check` -- takes <1 second. Set timeout to 15+ seconds.

### Testing
**IMPORTANT**: Tests interact with the DLT daemon and require specific setup.

- **Tests without DLT daemon**: 
  - Stop daemon: `killall --wait dlt-daemon || true`
  - Run specific test: `cargo test --test init_ok` -- takes 11 seconds. NEVER CANCEL. Set timeout to 60+ seconds.

- **Tests with DLT daemon**:
  - Start daemon: `sudo dlt-daemon -d` (requires sudo due to system permissions)
  - Run all tests: `cargo test` -- takes 60 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
  - Note: Tests display "FIFO /tmp/dlt cannot be opened" messages - this is expected behavior when daemon runs in restricted environment.

### Examples and Validation
- **Run example**: `cargo run --example simple` -- takes 10 seconds. Set timeout to 60+ seconds.
- **Test with console output**: `DLT_LOCAL_PRINT_MODE=FORCE_ON DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple`
- Expected output shows log messages with timestamps and log levels from TRACE to ERROR.

### Code Coverage
- **Coverage**: `cargo llvm-cov --fail-under-lines 80` -- takes 63 seconds. NEVER CANCEL. Set timeout to 180+ seconds.
- **Coverage report**: `./scripts/update-lcov.sh` -- takes 56 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- Generates `lcov.info` file for coverage analysis.

### Complete CI Validation
- **Full CI**: `./scripts/ci.sh` -- NOTE: Requires modification to use `sudo dlt-daemon -d` instead of `dlt-daemon -d` for proper daemon startup.
- The CI script performs all build, test, lint, and coverage steps in sequence.

## Validation Scenarios

ALWAYS test actual functionality after making changes:

1. **Basic functionality test**:
   - Ensure DLT daemon is running: `sudo dlt-daemon -d`
   - Run: `DLT_LOCAL_PRINT_MODE=FORCE_ON DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple`
   - Verify you see all 5 log levels (verbose, debug, info, warn, error) in console output

2. **Integration test**:
   - Run complete test suite: `cargo test`
   - Verify all 8 tests pass (2 unit tests + 5 integration tests + 1 log test)

3. **Code quality validation**:
   - Always run `cargo fmt -- --check` before committing
   - Always run `cargo clippy -- -D warnings` before committing
   - CI will fail if these checks don't pass

## Common Tasks

### Repository Structure
```
.
├── README.md              # Main documentation with usage examples
├── Cargo.toml            # Project configuration, dependencies, metadata
├── build.rs              # Bindgen configuration for FFI generation
├── src/
│   ├── lib.rs            # Main library code with DLT logger implementation
│   ├── libdlt.rs         # Generated FFI bindings (auto-generated)
│   └── libdlt_wrapper.h  # C header wrapper for bindgen
├── examples/
│   └── simple.rs         # Basic usage example showing all log levels
├── tests/                # Integration tests for initialization and logging
├── scripts/
│   ├── ci.sh             # Complete CI validation script
│   └── update-lcov.sh    # Coverage report generation
├── .github/workflows/
│   └── ci.yml            # GitHub Actions CI configuration
└── .devcontainer/        # Dev container setup for consistent environment
```

### Key Project Information
- **Language**: Rust (minimum version 1.82)
- **Dependencies**: libclang-dev (for bindgen), libdlt-dev (for DLT library), dlt-daemon (for testing)
- **Build system**: Cargo with custom build.rs for FFI binding generation
- **Testing**: Unit tests + integration tests requiring DLT daemon
- **Coverage requirement**: 80% line coverage (enforced in CI)
- **Documentation**: Available at [docs.rs/dlt_log](https://docs.rs/dlt_log)

### Environment-Specific Notes
- The DLT daemon may show "System not booted with systemd!" warnings - this is expected in containerized environments
- FIFO connection errors in logs are normal when daemon runs with restricted permissions
- Cross-compilation requires special setup with BINDGEN_EXTRA_CLANG_ARGS environment variable

### Build Artifacts
- **Documentation**: Generated in `target/doc/dlt_log/`
- **Coverage report**: Generated as `lcov.info` in repository root
- **Bindings**: Auto-generated `libdlt_bindings.rs` in build output directory

Always build and exercise your changes with the validation scenarios above before considering the task complete.