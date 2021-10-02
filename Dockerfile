FROM rust:1.55-slim-bullseye as build

RUN apt-get install -y openssl libssl-dev

COPY ./ ./

RUN cargo build --release

FROM rust:1.55-slim-bullseye

RUN apt-get install -y openssl libssl-dev

COPY --from=build ./target/release/rinter .

# set the startup command to run your binary
CMD ["./rinter"]