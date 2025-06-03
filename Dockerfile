FROM ubuntu:24.04

# Install essential packages
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo-watch for development (optional)
RUN cargo install cargo-watch

WORKDIR /app

# Copy only dependency files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy proto and build.rs for proto build
COPY src/proto src/proto
COPY build.rs ./

# Build dependencies (and proto)
RUN cargo build

# Copy the rest of the application (will be mounted in development)
COPY . .

# Build the application
RUN cargo build

# Command to run the application (can be overridden by docker-compose)
CMD ["cargo", "run", "--release"] 
