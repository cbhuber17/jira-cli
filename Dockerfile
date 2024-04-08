FROM rust:1.76.0-slim-buster as build

# Create a new empty shell project
RUN USER=root cargo new --bin jira-cli
WORKDIR /jira-cli

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/jira_cli*
RUN cargo build --release

# Final base
FROM rust:1.76

# Copy the build artifact from the build stage
COPY --from=build /jira-cli/target/release/jira-cli .

# Copy db
COPY ./data /data

# Set the startup command to run your binary
CMD ["./jira-cli"]