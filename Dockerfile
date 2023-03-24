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

FROM --platform=$BUILDPLATFORM alpine as deps
RUN apk add --update git
WORKDIR opt/deps
RUN git clone https://github.com/alpinelinux/aports --depth 1 && \
    export BARCH=aarch64 && \
    ls && \
    ls aports && \
    ls aports/scritps && \
    CBUILDROOT=~/sysroot-$BARCH aports/scripts/bootstrap.sh $BARCH
RUN git clone https://gitlab.alpinelinux.org/alpine/aports --depth 1
RUN cd aports && \
    git ckeckout 3.17.1 && \
    cd main/sqlite && \
    CTARGET=aarch64 abuild checksum && abuild -r
RUN ls

FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

FROM --platform=$BUILDPLATFORM rust:alpine3.17 as alpine-builder-arm64
COPY --from=xx / /
ARG TARGETPLATFORM
#RUN apk add --update git mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf
COPY --from=deps / /
WORKDIR /opt/lucle
COPY . . 
RUN xx-cargo build --release --verbose
RUN mv target/aarch64-unknown-linux-musl/release/lucle target/release/lucle

FROM alpine-builder-$TARGETARCH as build

FROM alpine:3.17 as alpine
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev
WORKDIR /opt/lucle
COPY --from=build /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080
CMD ["./lucle"]
