FROM rust:1.88.0-bookworm@sha256:5771a3cc2081935c59ac52b92d49c9e164d4fed92c9f6420aa8cc50364aead6e

RUN cargo install dioxus-cli --version 0.6.3 \
 && cargo install diesel_cli --version 2.2.7

COPY . /srv/app
WORKDIR /srv/app

RUN mkdir -p /.local/share/dioxus \
 && chown -R 1000:1000 /srv/app /usr/local/cargo /.local/share/dioxus

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

USER 1000:1000

CMD ["sleep", "infinity"]
