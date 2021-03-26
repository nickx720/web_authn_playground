# rust tooling is provided by `archlinux-rust`
FROM geal/archlinux-rust
MAINTAINER Geoffroy Couprie, contact@geoffroycouprie.com



ADD ./test-webauthn ./source
WORKDIR /source

EXPOSE 8080
RUN rustc -V

RUN cargo build --release

CMD cargo run