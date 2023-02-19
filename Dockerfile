FROM debian:bullseye-slim as debian-final
WORKDIR /opt/lucle
COPY target/release/lucle . 
RUN ls
CMD ["./lucle"]

FROM rust:alpine
#RUN apk add --update mysql mysql-client postgresql sqlite musl-dev protobuf

WORKDIR /opt/lucle

COPY . . 
RUN ls

#RUN cargo build --release --verbose

