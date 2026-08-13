FROM rust:1.97.1-trixie@sha256:3382bd20aa942806c533e9a73cd000474fb3ef173f71e684cc9b942675781769

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

# renovate: version=v2.3.12
RUN cargo install --locked \
    --git https://github.com/diesel-rs/diesel \
    --rev 9311439c61a0d5d35312990a889227a41e1f183d \
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
