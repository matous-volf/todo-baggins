FROM rust:1.96.1-trixie@sha256:1f0dbad1df66647807e6952d1db85d0b2bda7606cb2139d82517e4f009967376

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

# renovate: version=v2.3.11
RUN cargo install --locked \
    --git https://github.com/diesel-rs/diesel \
    --rev 07a80d4e9f32d62bec3d425d85c1ef5b8cddc80d \
    diesel_cli \
# renovate: version=v0.7.9
 && cargo install --locked \
    --git https://github.com/DioxusLabs/dioxus \
    --rev 3e43ffa6cf1488d5388c888eb5f2494a006ae6de \
    dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

CMD ["sleep", "infinity"]
