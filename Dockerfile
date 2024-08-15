FROM rust:latest AS build

# create a new empty shell project
RUN USER=root cargo new --bin tjm-backend
WORKDIR /tjm-backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/tjm_backend*
RUN cargo build --release

# our final base
FROM rust:latest

# copy the build artifact from the build stage
COPY --from=build /tjm-backend/target/release/tjm-backend .

# set the startup command to run your binary
CMD ["./tjm-backend"]

EXPOSE 8080