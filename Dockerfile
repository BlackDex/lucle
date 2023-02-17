FROM rust as backend 

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
