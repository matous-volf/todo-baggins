FROM rust:1.92.0-bookworm@sha256:9676d0547a259997add8f5924eb6b959c589ed39055338e23b99aba7958d6d31

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

RUN cargo install --git https://github.com/diesel-rs/diesel --rev 2e85ba060d3d70ea605ea58a79b8a435749a7adc --locked diesel_cli \
 && cargo install --git https://github.com/DioxusLabs/dioxus --rev 8f8b58ea80ba0ec8057807bcd58fb609f7a5f2b1 --locked dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

CMD ["sleep", "infinity"]
