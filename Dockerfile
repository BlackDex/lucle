FROM debian:bullseye-slim as debian-final
WORKDIR /opt/lucle
COPY target/release/lucle .
COPY web . 
EXPOSE 8080 
EXPOSE 3000
CMD ["./lucle"]

FROM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web && yarn build

FROM rust:alpine3.17 as alpine-builder
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite musl-dev protobuf

WORKDIR /opt/lucle

COPY . . 
RUN ls

RUN cargo build --release --verbose

FROM alpine:3.17 as alpine-final 
WORKDIR /opt/lucle
COPY --from=alpine-builder /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
CMD ["./lucle"]
