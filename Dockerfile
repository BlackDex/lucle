FROM rust as backend 
RUN sudo apt update
RUN sudo apt-get install -y mysql-server sqlite3 postgresql
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.12/protoc-21.12-linux-x86_64.zip && \
        unzip protoc-21.12-linux-x86_64.zip -d $HOME/.local && \
        export PATH="$PATH:$HOME/.local/bin"

COPY . . 
RUN cargo build --release

FROM node as frontend
COPY ./web .
RUN yarn && yarn build
 
FROM debian:bullseye-slim

RUN sudo apt update && sudo apt dist-upgrade -y

WORKDIR /opt/lucle

COPY --from=backend target/release/lucle . 
COPY --from=frontend web web/

EXPOSE 8080
EXPOSE 3000

CMD ["./lucle"] 
