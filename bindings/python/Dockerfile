FROM ubuntu:24.04

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/
ENV UV_LINK_MODE=copy

RUN mkdir /workspace
WORKDIR /workspace
COPY . .

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    cmake \
    clang-format \
    clang-tidy \
    cpplint \
    cppcheck \
    iwyu \
    libevdev-dev \
    make \
    pkg-config \
    git \
    && rm -rf /var/lib/apt/lists/* \
    && uv sync \
    # Install pre-commit hooks
    && uv run pre-commit install \
    # Shell completion
    && echo 'eval "$(uv generate-shell-completion bash)"' >> ~/.bashrc \
    # Auto activate venv
    && echo 'source /workspace/.venv/bin/activate' >> ~/.bashrc
