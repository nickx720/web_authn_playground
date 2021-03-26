FROM rust:1.50


COPY ./test-webauthn/src ./source/src
COPY ./test-webauthn/public ./source/public
COPY ./test-webauthn/Cargo.toml ./source
WORKDIR /source

EXPOSE 8080
RUN rustc -V

RUN cargo build

CMD cargo run