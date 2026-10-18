# base image pinned by digest, updated by dependabot
FROM mcr.microsoft.com/devcontainers/base:noble@sha256:21e9e3275736cb3bc33a2904b279a2ab292c51fedd53c2963463627d4586c3e6

# mount points for the Claude Code and GitHub CLI config volumes (see
# devcontainer.json), created here so fresh volumes are owned by vscode
# instead of root
RUN mkdir -p /home/vscode/.claude /home/vscode/.config/gh \
    && chown vscode:vscode /home/vscode/.claude /home/vscode/.config \
        /home/vscode/.config/gh

# DLT daemon, library and tools, bindgen requirements, pkg-config to find
# the DLT library and pip for pre-commit
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        dlt-daemon libdlt-dev dlt-tools libclang-dev pkgconf python3-pip \
    && rm -rf /var/lib/apt/lists/*

# cargo-llvm-cov for coverage measurement, pinned and checksum-verified
ARG LLVM_COV_VERSION=0.9.1
ARG LLVM_COV_SHA256=b3f68e625481fed9b16444174f3fa5ebcdbde4a1878803a35eabe2dcefcdc41a
RUN curl -LsSf -o /tmp/cargo-llvm-cov.tar.gz \
        "https://github.com/taiki-e/cargo-llvm-cov/releases/download/v${LLVM_COV_VERSION}/cargo-llvm-cov-x86_64-unknown-linux-gnu.tar.gz" \
    && echo "${LLVM_COV_SHA256}  /tmp/cargo-llvm-cov.tar.gz" | sha256sum --check --strict \
    && tar xzf /tmp/cargo-llvm-cov.tar.gz -C /usr/local/bin \
    && rm /tmp/cargo-llvm-cov.tar.gz

# pre-commit, pinned with hashes
COPY requirements.txt /tmp/requirements.txt
RUN pip3 install --break-system-packages --root-user-action=ignore \
        --require-hashes -r /tmp/requirements.txt \
    && rm /tmp/requirements.txt
