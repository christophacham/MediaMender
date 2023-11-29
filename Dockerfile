# =============== Builder ===============

FROM rust:1.74-alpine as builder
LABEL authors="JennyBlake"

# Add traget for static compilation
RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

# Empty shell project
RUN USER=root cargo new --bin mediamender
WORKDIR /mediamender

COPY ./Cargo.toml ./Cargo.toml

# Cache the dependencies, a docker thing
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs

# Continue as usual
COPY ./src  ./src

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/mediamender*
RUN cargo build --release --target x86_64-unknown-linux-musl

# =============== Runtime ===============

FROM alpine:latest

# Install libgcc, requirements for static linked RUST libs
RUN apk add --no-cache libgcc

# Create a user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the binary from the builder stage
COPY --from=builder /mediamender/target/x86_64-unknown-linux-musl/release/mediamender /usr/local/bin/mediamender

# Change the ownership of the binary
RUN chown appuser:appgroup /usr/local/bin/mediamender

# Switch the user
USER appuser

ENTRYPOINT ["mediamender"]