FROM rust:1.80-bookworm

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli diesel_cli
RUN apt-get update && apt-get install -y nodejs npm

COPY . /srv/app
WORKDIR /srv/app

RUN npm install

CMD npm run watch & dx serve --port 8000
