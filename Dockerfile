FROM debian:bullseye-slim

WORKDIR /opt/lucle

#COPY --from=backend target/release/lucle . 
#COPY --from=frontend /opt/lucle/web ./web
COPY . . 
RUN ls
EXPOSE 8080
EXPOSE 3000

CMD ["/bin/bash"] 
