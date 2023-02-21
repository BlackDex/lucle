FROM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web && yarn build

ARG ARCH=amd64

FROM rust:alpine3.17 as alpine-builder-amd64
ARG TARGETARCH
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose

FROM rust:alpine3.17 as alpine-builder-aarch64
ARG TARGETARCH
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose --target aarch64-unknown-linux-musl
RUN mv target/aarch64-unknown-linux-musl/release/lucle target/release/lucle

FROM alpine-builder-${ARCH} as build

FROM alpine:3.17 as alpine
WORKDIR /opt/lucle
COPY --from=build /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080
CMD ["./lucle"]
