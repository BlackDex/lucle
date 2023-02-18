FROM debian:bullseye-slim

WORKDIR /opt/lucle
COPY . . 
RUN ls


CMD ["/bin/bash"] 
