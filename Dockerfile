FROM rust:1.55-bullseye as build

COPY ./ ./

RUN cargo build --release

FROM rust:1.55-slim-bullseye

COPY --from=build ./target/release/rinter .

RUN apt-get update \
    && apt-get install -y ca-certificates

# set the startup command to run your binary
CMD ["./rinter"]