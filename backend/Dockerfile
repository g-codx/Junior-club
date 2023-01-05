FROM rust:1.64.0

RUN apt-get update -yqq && apt-get install -yqq cmake g++
RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./migrations ./migrations
COPY ./src ./src
COPY ./.env ./.env
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

WORKDIR backend/ ./

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]