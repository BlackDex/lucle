FROM --platform=$BUILDPLATFORM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web && yarn build

FROM rust:alpine3.17 as alpine-builder-amd64
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose

#FROM --platform=linux/arm64 rust:alpine3.17 as alpine-builder-arm64
#RUN apk add --update git mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
#WORKDIR /opt/lucle
#COPY . . 
#RUN CARGO_NET_GIT_FETCH_WITH_CLI=true cargo build --release --verbose
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:aarch64-musl as alpine-builder-arm64
RUN sudo apt update && \
    apt install -y protobuf-compiler mysql-client postgresql postgresql-client sqlite musl-dev 
WORKDIR /opt/speedupdate
COPY . .
RUN cargo build --release
RUN mv target/aarch64-unknown-linux-musl/release/lucle target/release/lucle

FROM alpine-builder-$TARGETARCH as build

FROM alpine:3.17 as alpine
RUN apk add --update mysql mysql-client postgresql postgresql-client sqlite 
WORKDIR /opt/lucle
COPY --from=build /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080
CMD ["./lucle"]
