# OFML Interpreter Development Container
FROM mcr.microsoft.com/devcontainers/rust:latest

# Install Node.js 24
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Claude Code globally
RUN npm install -g @anthropic-ai/claude-code

USER vscode
WORKDIR /workspace

CMD ["sleep", "infinity"]
