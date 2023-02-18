FROM rust:alpine
RUN apk update && \
    apk add mysql mysql-client postgresql sqlite musl-dev protobuf

WORKDIR /opt/lucle

COPY . . 

RUN cargo build --release --verbose

