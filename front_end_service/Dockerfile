FROM docker.io/rust:1.80.1-bullseye AS build

WORKDIR built_app

#COPY Carg.toml .
#COPY Cargo.lock .
#COPY Rocket.toml .
#COPY src .
COPY . .

RUN cargo build --release


FROM docker.io/debian:bullseye-20240812

WORKDIR /app

COPY --from=build /built_app/target/release/front_end_service .
COPY assets .
COPY templates .
COPY Rocket.toml .
