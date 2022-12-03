FROM debian:buster-slim

ENV ROCKET_ENV=production

RUN apt-get update && apt-get install -y curl
RUN curl -L \
    https://github.com/rcgoodfellow/pge-watch/releases/download/v0.1.0/pge-status-webserver \
    -o /usr/bin/pge-status-webserver
RUN chmod +x /usr/bin/pge-status-webserver

CMD pge-status-webserver
