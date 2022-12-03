FROM debian:buster-slim

ARG ARTIFACTS_URL=https://github.com/rcgoodfellow/pge-watch/releases/download
ARG VERSION=v0.1.0

ENV ROCKET_ENV=production

RUN apt-get update && apt-get install -y curl unzip
RUN curl -OL $ARTIFACTS_URL/$VERSION/pge-status-webserver
RUN curl -OL $ARTIFACTS_URL/$VERSION/templates.zip
RUN curl -OL $ARTIFACTS_URL/$VERSION/Rocket.toml
RUN unzip templates.zip
RUN chmod +x ./pge-status-webserver

CMD ./pge-status-webserver
