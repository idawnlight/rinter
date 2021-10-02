FROM rust:1.55-bullseye as build

COPY ./ ./

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=build ./target/release/rinter .

# set the startup command to run your binary
CMD ["./rinter"]