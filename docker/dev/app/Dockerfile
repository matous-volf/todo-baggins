FROM rust:1.80-bookworm

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli diesel_cli
RUN apt-get update && apt-get install -y nodejs npm supervisor

COPY . /srv/app
WORKDIR /srv/app

RUN npm install

COPY docker/dev/app/supervisord.conf /etc/supervisor/conf.d/supervisord.conf

CMD ["supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"]
