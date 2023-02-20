FROM node as build-frontend 
WORKDIR /opt/lucle
COPY . . 
RUN cd web && yarn
RUN cd web && yarn build

FROM rust:alpine3.17 as alpine-builder
RUN apk add --update mysql mysql-client mariadb-dev postgresql postgresql-client postgresql-dev sqlite sqlite-dev musl-dev protobuf

WORKDIR /opt/lucle

COPY . . 

RUN cargo build --release --verbose

FROM alpine:3.17 as alpine
WORKDIR /opt/lucle
COPY --from=alpine-builder /opt/lucle/target/release/lucle .
COPY --from=build-frontend /opt/lucle/web/dist ./web/dist
EXPOSE 3000
EXPOSE 8080

CMD ["./lucle"]
