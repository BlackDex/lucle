FROM debian:bullseye-slim

WORKDIR /opt/lucle
RUN rm ./*
COPY . . 
RUN ls


CMD ["/bin/bash"] 
