FROM --platform=$BUILDPLATFORM node as build-frontend 
WORKDIR /opt/lucle
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . . 
RUN cd web && pnpm install
RUN cd web && pnpm build

FROM --platform=$BUILDPLATFORM rust:alpine3.20 as alpine-builder-amd64
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose

FROM --platform=linux/arm64 rust:alpine3.20 as alpine-builder-arm64
RUN apk add --update mariadb-dev postgresql-dev sqlite-dev musl-dev
WORKDIR /opt/lucle
COPY . . 
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release --verbose

FROM alpine-builder-$TARGETARCH as build

FROM alpine:3.20 as alpine
WORKDIR /opt/lucle
#TODO: Workaround to fix link issue
RUN apk add mariadb-connector-c postgresql-client libgcc
COPY --from=build /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080
CMD ["./lucle"] 
