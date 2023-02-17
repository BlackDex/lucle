FROM ubuntu:22.04

RUN sudo apt update && sudo apt dist-upgrade -y

WORKDIR /opt/lucle

COPY target/release/lucle . 
COPY web web/

EXPOSE 8080
EXPOSE 3000

CMD ["./lucle"] 
