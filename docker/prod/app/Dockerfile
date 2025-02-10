# 1.83.0-bookworm
FROM rust@sha256:a45bf1f5d9af0a23b26703b3500d70af1abff7f984a7abef5a104b42c02a292b AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    nodejs=18.19.0+dfsg-6~deb12u2 \
    npm=9.2.0~ds1-1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/* \
 && cargo install dioxus-cli --version 0.6.3 \
 && cargo install diesel_cli --version 2.2.7

COPY . /srv/app
WORKDIR /srv/app

RUN npm install \
 && npm run build \
 && dx bundle

# bookworm
FROM debian@sha256:b877a1a3fdf02469440f1768cf69c9771338a875b7add5e80c45b756c92ac20a AS runner

RUN apt-get update \
 && apt-get install -y --no-install-recommends libpq5=15.10-0+deb12u1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /srv/app/target/dx/todo-baggins/release/web /srv/app
COPY .env /srv/app/.env

RUN chown -R 1000:1000 /srv/app

WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost || exit 1

USER 1000:1000

ENV IP="0.0.0.0"
ENV PORT="80"
CMD ["./server"]
