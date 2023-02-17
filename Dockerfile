FROM debian:bullseye as backend 
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.12/protoc-21.12-linux-x86_64.zip && \
        unzip protoc-21.12-linux-x86_64.zip -d $HOME/.local && \
        export PATH="$PATH:$HOME/.local/bin"
RUN apt-get update
RUN apt get install -y rust
WORKDIR /opt/lucle
COPY . . 
RUN cargo build --release --verbose 

FROM node as frontend
WORKDIR /opt/lucle
COPY ./web ./web
RUN cd web && yarn && yarn build
 
FROM debian:bullseye-slim

WORKDIR /opt/lucle

COPY --from=backend target/release/lucle . 
COPY --from=frontend /opt/lucle/web ./web

EXPOSE 8080
EXPOSE 3000

CMD ["./lucle"] 
