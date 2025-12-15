# Contributing Guide
Thank you for considering contributing to the project.

## Getting started
To get started, fork the repository and clone it to your local machine.
After implementation, raise a pull request to the main repository.

## Setup environment
This project uses a devcontainer to define the build environment.

You can use GitHub Codespaces or the Visual Studio Code Remote - Containers extension to open the project in a containerized environment.

In case you want to set up the environment manually, you need to have a standard Rust development environment.
Additionally, please install the tools listed in [onCreateCommand.sh](.devcontainer/onCreateCommand.sh).

## Building and testing
Use standard cargo commands to build and test the project.

`bindgen` is used to generate the FFI bindings to the DLT library.
In case you need access to additional functions from the DLT library, you need to update the `build.rs` script.
See `allowlist_item` in the `bindgen` call.

To test the connection to the DLT system, you need to start the DLT daemon.
In the devcontainer, this is done automatically when you start the container, see [postStartCommand.sh](.devcontainer/postStartCommand.sh).
To manually start the DLT daemon, run `dlt-daemon -d`.

Full CI tests can be executed by calling the script `./scripts/ci.sh`.

## Code coverage
To update the code coverage data, run `./scripts/coverage.sh`.
In Visual Studio Code, you can use the `ryanluker.vscode-coverage-gutters` extension to display the coverage information in the editor.
Or open `./target/coverage/html/index.html` in browser.
