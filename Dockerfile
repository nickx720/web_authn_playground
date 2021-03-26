FROM rust:1.50


COPY ./test-webauthn/src ./source
COPY ./test-webauthn/public ./source
WORKDIR /source

EXPOSE 8080
RUN rustc -V

RUN cargo build

CMD cargo run