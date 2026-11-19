FROM rust:1.99.1-trixie@sha256:09027f873d45b8ecd1dc86f1fa13b7a62b875c961f017a0e331267b350430137

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

# hadolint ignore=DL3066
USER app_user

# renovate: version=v2.4.0
RUN cargo install --locked \
    --git https://github.com/diesel-rs/diesel \
    --rev 48acdc824d1eb93beaf169541b8dab3d3e5eaf3c \
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
