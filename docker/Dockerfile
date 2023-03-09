# sample dockerfile

# we use it as `disque-exporter/Dockerdile` together with this `docker-comopose.yml`
# snippet:
#
#    version: "3"
#    services:
#      disque:
#        image: richnorth/disque
#        expose:
#          - "7711"
#        restart: always
#      disque_exporter:
#        build: ./disque-exporter
#        environment:
#          - DISQUE_EXPORTER_LISTEN_ADDR=[::]:9599
#          - DISQUE_EXPORTER_DISQUE_URL=redis://disque:7711
#          - DISQUE_EXPORTER_HOST=foo
#        restart: always
#        ports:
#          - "<some-address-that-prometheus-can-reach>:9599:9599"

FROM rust AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo install --target x86_64-unknown-linux-musl --git https://github.com/kaspar030/disque-exporter

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/disque-exporter /
USER 1000
CMD ["/disque-exporter"]
