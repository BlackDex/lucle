FROM rust:alpine
RUN apk add --update mysql mysql-client postgresql sqlite musl-dev protobuf

WORKDIR /opt/lucle

COPY . . 

RUN cargo build --release --verbose

