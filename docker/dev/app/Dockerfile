FROM rust:1.98.1-trixie@sha256:a8a5f0a1e5fe7dfe1d352591e4a1c7dd2c08fd70475cae872cf3458ba0df0546

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

# renovate: version=v2.3.11
RUN cargo install --locked \
    --git https://github.com/diesel-rs/diesel \
    --rev 07a80d4e9f32d62bec3d425d85c1ef5b8cddc80d \
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
