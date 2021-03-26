FROM rust:1.50


ADD ./test-webauthn ./source
WORKDIR /source

EXPOSE 8080
RUN rustc -V

RUN cargo build

CMD cargo run