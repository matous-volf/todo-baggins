FROM rust:1.80-bookworm

RUN rustup target add wasm32-unknown-unknown && \
    cargo install dioxus-cli diesel_cli && \
    apt-get update && apt-get install -y nodejs=18.19.0+dfsg-6~deb12u2 npm=9.2.0~ds1-1 supervisor=4.2.5-1

COPY . /srv/app
WORKDIR /srv/app

RUN npm install

COPY docker/dev/app/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

HEALTHCHECK CMD curl --fail http://localhost:8000 || exit 1
USER root
CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
