FROM debian:bullseye-slim

WORKDIR /opt/lucle
RUN rm -rf .*
COPY . . 
RUN ls


CMD ["/bin/bash"] 
