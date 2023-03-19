FROM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web && yarn build

FROM rust:alpine3.17 as alpine-builder-amd64
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose

FROM --platform=$BUILDPLATFORM rust:alpine3.17 as alpine-builder-arm64
RUN apk add --update gcc-cross-embedded mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
RUN rustup target add aarch64-unknown-linux-musl
WORKDIR /opt/lucle
COPY . . 
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
RUN cargo build --release --verbose --target aarch64-unknown-linux-musl
RUN mv target/aarch64-unknown-linux-musl/release/lucle target/release/lucle

FROM alpine-builder-$TARGETARCH as build

FROM alpine:3.17 as alpine
WORKDIR /opt/lucle
COPY --from=build /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080
CMD ["./lucle"]
