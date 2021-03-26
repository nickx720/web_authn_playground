# rust tooling is provided by `archlinux-rust`
FROM geal/archlinux-rust
MAINTAINER Geoffroy Couprie, contact@geoffroycouprie.com

# needed by rust
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib

ADD ./test-webauthn ./source
WORKDIR ./source

EXPOSE 8080
RUN rustc -V

RUN cargo build --release

CMD cargo run