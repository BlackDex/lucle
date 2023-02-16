ARG ARCH=amd64

FROM messense/rust-musl-cross:x86_64-musl as amd64
RUN sudo apt update && \
    apt install -y libssl-dev protobuf-compiler mysql-server sqlite3 postgresql

WORKDIR /opt/lucle

COPY . .

RUN cargo build --release
RUN mv target/x86_64-unknown-linux-musl/release/lucle target/release/lucle

FROM messense/rust-musl-cross:aarch64-musl as arm64

RUN sudo apt update && \
    apt install -y libssl-dev protobuf-compiler

WORKDIR /opt/lucle

COPY . .

RUN cargo build --release
RUN mv target/aarch64-unknown-linux-musl/release/lucle target/release/lucle

FROM node as build-frontend

WORKDIR /opt/lucle
COPY web/ . 
RUN yarn && yarn build 

FROM ${ARCH} as build

FROM alpine:3.17 as final
ARG TARGETARCH
RUN apk upgrade

WORKDIR /opt/lucle

COPY --from=build /opt/lucle/target/release/lucle ./lucle
COPY --from=build-frontend /opt/lucle/ ./web
EXPOSE 8080
EXPOSE 3000

CMD ["./lucle"] 
