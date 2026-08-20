FROM rust:1.97.1-trixie@sha256:3382bd20aa942806c533e9a73cd000474fb3ef173f71e684cc9b942675781769 AS builder_base

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
    # renovate: repology=debian_13/openjdk-25
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


FROM debian:trixie@sha256:34cd9e9fd437c0a095ec39cb2e73422c9f30821b0d0848ed74fd0d43bae4d958 AS runner_web

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
 # renovate: repology=debian_13/postgresql-17
 libpq5=17.11-0+deb13u1 \
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
