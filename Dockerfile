FROM debian:bullseye-slim as debian-final
WORKDIR /opt/lucle
COPY . . 
RUN ls
CMD ["./lucle"]

FROM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web & & yarn build

FROM rust:alpine as alpine-builder
RUN apk add --update mysql mysql-client postgresql sqlite musl-dev protobuf

WORKDIR /opt/lucle

COPY . . 
RUN ls

RUN cargo build --release --verbose

