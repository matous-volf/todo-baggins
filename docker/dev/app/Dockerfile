FROM rust:1.92.0-bookworm@sha256:e90e846de4124376164ddfbaab4b0774c7bdeef5e738866295e5a90a34a307a2

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

RUN cargo install --git https://github.com/diesel-rs/diesel --rev 7b4f88861420ba7902f2cf7ef7f7c36a0e34face --locked diesel_cli \
 && cargo install --git https://github.com/DioxusLabs/dioxus --rev 22b06badde44ba1af0fcf339c91b66483175b660 --locked dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

CMD ["sleep", "infinity"]
