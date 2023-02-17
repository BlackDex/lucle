FROM debian:bullseye as backend 
RUN apt-get update
RUN apt-get install -y curl zip bash gcc
WORKDIR /opt/lucle
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y 
ENV PATH="/root/.cargo/bin:${PATH}" 
RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v21.12/protoc-21.12-linux-x86_64.zip && \
        unzip protoc-21.12-linux-x86_64.zip -d /usr/bin/local
#ENV PATH="$HOME/.local/bin:${PATH}" 
RUN protoc --help
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
