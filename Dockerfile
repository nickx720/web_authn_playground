FROM geal/archlinux-rust
MAINTAINER Geoffroy Couprie, contact@geoffroycouprie.com

# needed by rust
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib


COPY ./test-webauthn/src ./source
COPY ./test-webauthn/public ./source
COPY ./test-webauthn/Cargo.toml ./source
WORKDIR /source

EXPOSE 8080
RUN rustc -V

RUN cargo build

CMD cargo run