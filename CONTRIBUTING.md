# Contributing Guide
Thank you for considering contributing to the project.

## Getting started
To get started, fork the repository and clone it to your local machine.
After implementation, raise a pull request to the main repository.

## Setup environment
This project uses a devcontainer to define the build environment.

You can use GitHub Codespaces or the Visual Studio Code Remote - Containers extension to open the project in a containerized environment.

In case you want to set up the environment manually, you need to have a standard Rust development environment.
Additionally, please install the tools listed in the [Dockerfile](.devcontainer/Dockerfile).
Code coverage requires a nightly toolchain with the `llvm-tools` component, see the rust feature in [devcontainer.json](.devcontainer/devcontainer.json).

## Building and testing
Use standard cargo commands to build and test the project.

`bindgen` is used to generate the FFI bindings to the DLT library.
In case you need access to additional functions from the DLT library, you need to update the `build.rs` script.
See `allowlist_item` in the `bindgen` call.

To test the connection to the DLT system, you need to start the DLT daemon.
In the devcontainer, this is done automatically when you start the container, see [postStartCommand.sh](.devcontainer/postStartCommand.sh).
To manually start the DLT daemon, run `./scripts/start-dlt-daemon.sh`. It only accepts connections from localhost.
Tests that check the received messages with `dlt-receive` skip the check if the daemon is not running.
Set `DLT_LOG_REQUIRE_DAEMON=1` to make them fail instead, as `./scripts/ci.sh` and `./scripts/coverage.sh` do.

Full CI tests can be executed by calling the script `./scripts/ci.sh`.

The devcontainer uses the DLT version packaged in Ubuntu.
To test against another version, run `./scripts/install-dlt-daemon.sh <revision>` with a git revision of [COVESA/dlt-daemon](https://github.com/COVESA/dlt-daemon), e.g. `v3.0.0` or `master`.
It removes the DLT packages and installs a build of that revision instead; rebuild the container to get the packaged version back.
CI does this for the versions listed in [dlt-versions.yml](.github/workflows/dlt-versions.yml), pinned to the commit hashes of their tags, and nightly for `master`.
When adding a version there, also add it to the list of supported DLT versions in [README.md](README.md).

## Code coverage
To update the code coverage data, run `./scripts/coverage.sh`.
In Visual Studio Code, you can use the `ryanluker.vscode-coverage-gutters` extension to display the coverage information in the editor.
Or open `./target/coverage/html/index.html` in browser.
