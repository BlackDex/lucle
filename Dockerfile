FROM --platform=$BUILDPLATFORM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web && yarn build


FROM messense/rust-musl-cross:x86_64-musl as builder-amd64
RUN sudo apt update && sudo apt install -y mariadb-server mariadb-client postgresql postgresql-client sqlite protobuf-compiler
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose

FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:aarch64-musl as builder-arm64
RUN sudo apt update && \
    apt install -y protobuf-compiler mariadb-client mariadb-server postgresql postgresql-client sqlite 
WORKDIR /opt/speedupdate
COPY . .
RUN cargo build --release
RUN mv target/aarch64-unknown-linux-musl/release/lucle target/release/lucle

FROM builder-$TARGETARCH as builder

FROM alpine:3.17 as alpine
WORKDIR /opt/lucle
COPY --from=builder /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080
CMD ["./lucle"]
