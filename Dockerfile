FROM rust:1.50

# needed by rust
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

COPY ./test-webauthn/src ./source/src
COPY ./test-webauthn/public ./source/public
COPY ./test-webauthn/Cargo.toml ./source
WORKDIR /source

EXPOSE 8080
RUN rustc -V

RUN cargo build

CMD cargo run