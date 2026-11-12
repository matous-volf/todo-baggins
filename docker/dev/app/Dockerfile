FROM rust:1.99.1-trixie@sha256:8a4e90c7946b74626488b1446f712c0fe0ed1a596cbd3d871684bc9c5433ae07

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

# hadolint ignore=DL3066
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

HEALTHCHECK CMD ["curl", "--fail", "-H", "Accept: text/html", "http://localhost:8000"]

CMD ["sleep", "infinity"]
