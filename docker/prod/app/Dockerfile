FROM rust:1.96.1-trixie@sha256:1f0dbad1df66647807e6952d1db85d0b2bda7606cb2139d82517e4f009967376 AS builder_base

RUN cargo install --git https://github.com/diesel-rs/diesel --rev 7b4f88861420ba7902f2cf7ef7f7c36a0e34face --locked diesel_cli \
 && cargo install --git https://github.com/DioxusLabs/dioxus --rev 3e43ffa6cf1488d5388c888eb5f2494a006ae6de --locked dioxus-cli --features disable-telemetry

COPY . /srv/app
WORKDIR /srv/app


FROM builder_base AS builder_android

ARG ANDROID_NDK_VERSION=26.2.11394342
ARG ANDROID_COMMAND_LINE_TOOLS_VERSION=13114758
ARG ANDROID_COMMAND_LINE_TOOLS_SHA256=7ec965280a073311c339e571cd5de778b9975026cfcbe79f2b1cdcb1e15317ee
ARG ANDROID_VERSION=34
ARG ANDROID_BUILD_TOOLS_VERSION=34.0.0
ARG ANDROID_BUNDLETOOL_VERSION=1.18.3
ARG ANDROID_BUNDLETOOL_SHA256=a099cfa1543f55593bc2ed16a70a7c67fe54b1747bb7301f37fdfd6d91028e29
ENV ANDROID_SDK_ROOT=/opt/android-sdk

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update && apt-get install -y --no-install-recommends \
    # renovate: repology=debian_13/openjdk-25-jdk-headless
    openjdk-25-jdk-headless=25.0.3+9-2~deb13u1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/* \
 && rustup target add aarch64-linux-android \
 && mkdir -p "$ANDROID_SDK_ROOT/cmdline-tools" \
 && curl -fsSL -o /tmp/cmdline-tools.zip "https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_COMMAND_LINE_TOOLS_VERSION}_latest.zip" \
 && unzip -q /tmp/cmdline-tools.zip -d "$ANDROID_SDK_ROOT/cmdline-tools" \
 && export PATH="$ANDROID_SDK_ROOT/cmdline-tools/latest/bin:$ANDROID_SDK_ROOT/platform-tools:$PATH" \
 && mv "$ANDROID_SDK_ROOT/cmdline-tools/cmdline-tools" "$ANDROID_SDK_ROOT/cmdline-tools/latest" \
 && rm /tmp/cmdline-tools.zip \
 # `yes` can exit with 141, which is okay, so the `test` is added to ignore it.
 && yes | sdkmanager --sdk_root="$ANDROID_SDK_ROOT" --licenses > /dev/null; test "${PIPESTATUS[1]}" -eq 0 \
 && sdkmanager --sdk_root="$ANDROID_SDK_ROOT" "platform-tools" "platforms;android-$ANDROID_VERSION" "build-tools;$ANDROID_BUILD_TOOLS_VERSION" "ndk;$ANDROID_NDK_VERSION" \
 && curl -fsSL -o /tmp/bundletool-all.jar "https://github.com/google/bundletool/releases/download/$ANDROID_BUNDLETOOL_VERSION/bundletool-all-${ANDROID_BUNDLETOOL_VERSION}.jar" \
 && echo "$ANDROID_BUNDLETOOL_SHA256 /tmp/bundletool-all.jar" | sha256sum -c - \
 && keytool -genkeypair -noprompt -keystore /tmp/android_keystore.jks -alias key -keyalg RSA -keysize 2048 -validity 3660 -dname "CN=" -storepass 123456 -keypass 123456 \
 && export ANDROID_HOME="$ANDROID_SDK_ROOT" \
 && export ANDROID_NDK_HOME="$ANDROID_SDK_ROOT/ndk/$ANDROID_NDK_VERSION" \
 && dx bundle --locked --platform android --target aarch64-linux-android --release \
 && java -jar /tmp/bundletool-all.jar build-apks --bundle=/srv/app/target/dx/todo_baggins/release/android/app/app/build/outputs/bundle/release/TodoBaggins-aarch64-linux-android.aab --output=/tmp/todo_baggins.apks --mode=universal --ks=/tmp/android_keystore.jks --ks-key-alias=key --ks-pass=pass:123456 \
 && mkdir -p /srv/app/bundle \
 && unzip -qp /tmp/todo_baggins.apks universal.apk > /srv/app/bundle/todo_baggins.apk


FROM builder_base AS builder_web

RUN dx bundle --locked --release


FROM debian:trixie@sha256:fac46bff2e02f51425b6e33b0e1169f55dfb053d83511ca28aa50c09fd5ed7a4 AS runner_web

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
 # renovate: repology=debian_13/libpq5
 libpq5=17.10-0+deb13u1 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder_web /srv/app/target/dx/todo_baggins/release/web /srv/app/web

RUN chown -R 1000:1000 /srv/app/web

WORKDIR /srv/app/web

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost || exit 1

USER 1000:1000

ENV IP="0.0.0.0"
ENV PORT="80"
CMD ["./server"]
