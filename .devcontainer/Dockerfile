# base image pinned by digest, updated by dependabot
FROM mcr.microsoft.com/devcontainers/base:noble@sha256:d7c468679f45a52ad3673d06656b5bf16e488990b17216a1fa295d9e1f89d724

# mount points for the Claude Code and GitHub CLI config volumes (see
# devcontainer.json), created here so fresh volumes are owned by vscode
# instead of root
RUN mkdir -p /home/vscode/.claude /home/vscode/.config/gh \
    && chown vscode:vscode /home/vscode/.claude /home/vscode/.config \
        /home/vscode/.config/gh
