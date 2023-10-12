FROM rust:latest as build
RUN apt-get update && apt-get install -y \
    libpq-dev

COPY ./project .

RUN cargo build --release

FROM rust:latest

WORKDIR /project

COPY --from=build /target/release/project project


ENTRYPOINT ["./project"]