FROM rust:1.97.1-trixie@sha256:b1b3c9c0d921d7fa0a6d1f9ec7e4eab87f8c8ec97644c3d791450f131dec813f

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

# renovate: version=v2.3.14
RUN cargo install --locked \
    --git https://github.com/diesel-rs/diesel \
    --rev 4007d771b1e808839f771a693f6bae9f821f0e64 \
    diesel_cli \
# renovate: version=v0.7.10
 && cargo install --locked \
    --git https://github.com/DioxusLabs/dioxus \
    --rev 57d6794ad60b949e5bd8aa282f6f8c3dc97a365e \
    dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

CMD ["sleep", "infinity"]
