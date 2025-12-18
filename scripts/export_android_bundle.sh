#!/bin/sh

mkdir -p ../bundle \
&& docker compose -f docker-compose-prod.yaml cp app:/srv/app/android bundle
