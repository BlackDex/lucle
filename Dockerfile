FROM rust:alpine
RUN apk update && \
    apk add mysql mysql-client postgresql musl-dev 

WORKDIR /opt/lucle

COPY . . 

RUN cargo build --release --verbose

