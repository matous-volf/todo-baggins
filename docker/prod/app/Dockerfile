FROM rust:1.80-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown && \
    cargo install dioxus-cli diesel_cli && \
    apt-get update && apt-get install -y nodejs=18.19.0+dfsg-6~deb12u2 npm=9.2.0~ds1-1 supervisor=4.2.5-1

COPY . /srv/app
WORKDIR /srv/app

RUN npm install
RUN npm run build
RUN dx build --release

FROM debian:bookworm-slim AS runner

RUN apt-get update && apt-get install -y libpq5=15.8-0+deb12u1

COPY --from=builder /srv/app/dist /srv/app/dist
COPY .env /srv/app/.env

RUN chown -R 1000:1000 /srv/app

WORKDIR /srv/app

HEALTHCHECK CMD curl --fail http://localhost:8000 || exit 1

USER 1000:1000

CMD ["./dist/todo-baggins"]
