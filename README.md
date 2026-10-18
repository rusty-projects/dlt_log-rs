# log adapter for DLT

[![Build status](https://github.com/rusty-projects/dlt_log-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/rusty-projects/dlt_log-rs/actions)
[![DLT compatibility](https://github.com/rusty-projects/dlt_log-rs/actions/workflows/dlt-versions.yml/badge.svg?branch=main)](https://github.com/rusty-projects/dlt_log-rs/actions/workflows/dlt-versions.yml)
[![Crates.io](https://img.shields.io/crates/v/dlt_log.svg)](https://crates.io/crates/dlt_log)
[![Documentation](https://docs.rs/dlt_log/badge.svg)](https://docs.rs/dlt_log)

This crate is a [`log`](https://docs.rs/log) adapter for the Diagnostic Log and Trace (DLT) system, which is widely used in the automotive industry for logging and tracing in embedded systems.
It registers your application and context with the DLT daemon and sends the messages of the `log` macros to DLT.
For more information about DLT, see the [DLT project on GitHub](https://github.com/COVESA/dlt-daemon).

## Usage

Add the `dlt_log` crate to your `Cargo.toml`:

```toml
[dependencies]
log = { version = "0.4", features = ["std"] }
dlt_log = "1"
```

Initialize it once with your application and context information, then use the `log` macros:

```rust
fn main() {
    let _ = dlt_log::init("TEST", "Rust tests", "EXPL", "Smoke test example");

    log::trace!("Tracing the untraceable!");
    log::debug!("Debugging the debugger!");
    log::info!("Information overload: {} + {} = {}", 2, 2, 4);
    log::warn!("Warning: Low on coffee!");
    log::error!("Error: Something went terribly right!");
}
```

The build needs `libclang-dev` for the binding generation and `libdlt-dev` to link against the DLT library, see [Build requirements](#build-requirements).

## Viewing logs

Connect the DLT viewer to the DLT daemon to view the log messages.

![Example logs in DLT viewer](https://raw.githubusercontent.com/rusty-projects/dlt_log-rs/main/doc/dlt-viewer-example.png)

Log levels and logging backend are configured by the DLT system, see the [DLT documentation](https://github.com/COVESA/dlt-daemon/blob/master/doc/dlt_for_developers.md).
The default log level is `info`, so only messages with level `info` or higher are logged.
For example, to enable all log levels, set the `DLT_INITIAL_LOG_LEVEL` environment variable to `::6`.
To print the messages to the console in addition, set `DLT_LOCAL_PRINT_MODE` to `FORCE_ON`:

```bash
DLT_LOCAL_PRINT_MODE=FORCE_ON DLT_INITIAL_LOG_LEVEL="::6" cargo run --example simple
```

```text
2025/12/11 18:13:51.456738   57739781 000 ECU1 TEST EXPL log verbose V 1 [[simple.rs:5] Tracing the untraceable!]
2025/12/11 18:13:51.456809   57739782 001 ECU1 TEST EXPL log debug V 1 [[simple.rs:6] Debugging the debugger!]
2025/12/11 18:13:51.456888   57739783 002 ECU1 TEST EXPL log info V 1 [[simple.rs:7] Information overload: 2 + 2 = 4]
2025/12/11 18:13:51.456931   57739783 003 ECU1 TEST EXPL log warn V 1 [[simple.rs:8] Warning: Low on coffee!]
2025/12/11 18:13:51.456989   57739784 004 ECU1 TEST EXPL log error V 1 [[simple.rs:9] Error: Something went terribly right!]
```

## Build requirements

At least Rust 1.82 is required, as the latest `bindgen` does not support older versions, see [bindgen issue #3052](https://github.com/rust-lang/rust-bindgen/issues/3052).

[`bindgen`](https://docs.rs/bindgen) generates the FFI bindings to the DLT C library during the build, so that they match the headers of your DLT library.
It needs `libclang-dev`, and linking needs `libdlt-dev`. On Ubuntu:

```bash
sudo apt-get install libclang-dev libdlt-dev
```

### Cross-compilation

As an example, assume a yocto SDK for aarch64 that includes the DLT library is installed at `/opt/poky/5.1.2`:
- `/opt/poky/5.1.2/environment-setup-cortexa57-poky-linux`
- `/opt/poky/5.1.2/sysroots/cortexa57-poky-linux/usr/lib/libdlt.so`
- `/opt/poky/5.1.2/sysroots/cortexa57-poky-linux/usr/include/dlt/dlt.h`

Create `.cargo/config.toml` in your project directory with the following content:

```toml
[target.aarch64-unknown-linux-gnu]
linker="/opt/poky/5.1.2/sysroots/x86_64-pokysdk-linux/usr/bin/aarch64-poky-linux/aarch64-poky-linux-gcc"
rustflags = [
"-C", "link-arg=--sysroot=/opt/poky/5.1.2/sysroots/cortexa57-poky-linux",
]
```

Bindgen needs the sysroot to find the DLT headers, so set `BINDGEN_EXTRA_CLANG_ARGS` before building:

```bash
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=/opt/poky/5.1.2/sysroots/cortexa57-poky-linux"
cargo build --release --target aarch64-unknown-linux-gnu
```

## Supported DLT versions

CI tests this crate against these versions of [dlt-daemon](https://github.com/COVESA/dlt-daemon), using library, daemon and `dlt-receive` of the same version:

| DLT version | Notes |
|-------------|-------|
| v2.18.9     | |
| v2.18.10    | also tested as Ubuntu 24.04 package |
| v2.18.11    | tested with deadlock fix, see below |
| v3.0.0      | tested with deadlock fix, see below |

`master` is tested nightly to detect upcoming changes; it is not a supported version and not part of the badge.
Version 3 of the DLT library is not binary compatible with version 2, so rebuild your application when you switch.

The DLT library of v2.18.11 and v3.0.0 can hang when the application exits, independent of this crate: its exit handler `dlt_free()` deadlocks with its housekeeper thread.
Applications that exit shortly after logging are affected most, with v2.18.11 nearly always.
The bug is fixed upstream in [dlt-daemon PR #915](https://github.com/COVESA/dlt-daemon/pull/915), which is not released yet.
CI applies the relevant part of the fix, [`dlt-daemon-free-deadlock.patch`](https://github.com/rusty-projects/dlt_log-rs/blob/main/scripts/patches/dlt-daemon-free-deadlock.patch); apply it to your DLT library as well if you use one of these versions.

## Logging untrusted data

Log messages are passed to DLT as they are, without escaping control characters.
Inside the DLT protocol this is harmless, as every message has an explicit length.
However, tools that print messages as text, e.g. the console output with `DLT_LOCAL_PRINT_MODE=FORCE_ON` or `dlt-receive`, print most control characters unchanged.
Line breaks are replaced by spaces, but ANSI escape sequences are not.
Untrusted data in a log message can therefore inject terminal escape sequences, e.g. to hide or alter what is displayed.
If you log untrusted data, escape it before logging, e.g. by using the `{:?}` format specifier:

```rust
let user_input = "name\x1b[2K\x1b[1Gfaked content";
log::info!("User name: {:?}", user_input);
```

## Message length limit

DLT limits the size of a log message payload, by default to 1390 bytes (`DLT_USER_BUF_MAX_SIZE`).
The limit applies to the complete payload, which also contains the `[file:line] ` prefix added by this crate and a few bytes of DLT argument header.
Longer messages are truncated by the DLT library, which appends `... <<Message truncated, too long>>`.
The full message is still formatted before it is truncated.

The limit can be changed with the `DLT_LOG_MSG_BUF_LEN` environment variable, e.g. `DLT_LOG_MSG_BUF_LEN=4096`.
To log large data reliably, split it into several log messages, e.g. for ASCII data:

```rust
let data = "x".repeat(5000);
for (i, chunk) in data.as_bytes().chunks(1000).enumerate() {
    log::info!("data part {}: {}", i, String::from_utf8_lossy(chunk));
}
```

## Errors

[`init`] returns an [`InitializeError`] if the initialization fails, because a parameter contains a NUL byte, another logger is set already, or the DLT library returns an error.

[`init`]: https://docs.rs/dlt_log/latest/dlt_log/fn.init.html
[`InitializeError`]: https://docs.rs/dlt_log/latest/dlt_log/enum.InitializeError.html

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/rusty-projects/dlt_log-rs/blob/main/LICENSE) file for details.

License information for DLT daemon and library can be found at [Diagnostic Log and Trace](https://github.com/COVESA/dlt-daemon#license).
