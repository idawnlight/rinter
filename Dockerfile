FROM rust:1.55-slim-bullseye as build

COPY ./ ./

RUN cargo build --release

FROM rust:1.55-slim-bullseye

COPY --from=build ./target/release/rinter .

# set the startup command to run your binary
CMD ["./rinter"]