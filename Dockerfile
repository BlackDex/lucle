FROM rust:alpine

WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose

