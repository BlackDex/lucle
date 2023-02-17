FROM debian:bullseye-slim

RUN apk upgrade

WORKDIR /opt/lucle

COPY target/release/lucle lucle 
COPY web web

EXPOSE 8080
EXPOSE 3000

CMD ["./lucle"] 
