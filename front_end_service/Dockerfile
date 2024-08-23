FROM docker.io/rust:1.80.1-bullseye AS build

WORKDIR built_app

COPY Cargo.toml .
COPY Cargo.lock .
COPY Rocket.toml .
COPY src src
COPY assets assets
COPY templates templates

RUN cargo build --release

FROM docker.io/debian:bullseye-20240812

WORKDIR /app

COPY --from=build /built_app/target/release/front_end_service .
COPY assets assets
COPY templates templates
COPY Rocket.toml .
