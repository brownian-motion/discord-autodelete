FROM rust:1.74 AS builder

# 1 create a new empty shell project
RUN USER=root cargo new --bin discord-autodelete
WORKDIR /discord-autodelete

# 2 copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3 cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# 4 copy source tree
COPY ./src ./src
# The last modified attribute of main.rs needs to be updated manually,
# otherwise cargo won't rebuild it.
RUN touch -a -m ./src/main.rs

# 5 build for release
RUN cargo build --release

#########################

FROM rust:1.74
COPY --from=builder /discord-autodelete/target/release/discord-autodelete /bin/discord-autodelete
WORKDIR /app
CMD [ "/bin/discord-autodelete" ]