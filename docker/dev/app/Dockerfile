FROM rust:1.96.1-trixie@sha256:1f0dbad1df66647807e6952d1db85d0b2bda7606cb2139d82517e4f009967376

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

RUN cargo install --git https://github.com/diesel-rs/diesel --rev 7b4f88861420ba7902f2cf7ef7f7c36a0e34face --locked diesel_cli \
 && cargo install --git https://github.com/DioxusLabs/dioxus --rev 3e43ffa6cf1488d5388c888eb5f2494a006ae6de --locked dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

CMD ["sleep", "infinity"]
