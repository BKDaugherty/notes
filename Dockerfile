# Dockerfile for creating a statically-linked Rust application using docker's
# multi-stage build feature. This also leverages the docker build cache to avoid
# re-downloading dependencies if they have not changed.
FROM clux/muslrust:stable AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new notes
WORKDIR /usr/src/notes

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy the source and build the application.
COPY src ./src

RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path . --root /usr/local/

# Copy the statically-linked binary into a scratch container.
FROM debian:jessie
RUN apt-get update && apt-get install -y libmysqlclient-dev
COPY --from=build /usr/local/bin/notes .
USER 1000
CMD ["./notes"]