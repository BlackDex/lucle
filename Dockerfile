FROM debian:bullseye-slim

WORKDIR /opt/lucle

#%COPY --from=backend target/release/lucle . 
COPY --from=frontend /opt/lucle/web ./web

EXPOSE 8080
EXPOSE 3000

CMD ["./lucle"] 
