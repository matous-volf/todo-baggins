FROM rust:1.99.0-trixie@sha256:81015672d6939b0265f09adf96d5de125482bb1330106076956b0518e74edc90

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
