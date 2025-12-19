#!/bin/sh

echo "Warning: This script needs to stop any currently running Docker compose stacks. Stopping them now..."
docker compose down

mkdir -p bundle/android bundle/temp \
&& docker compose -f docker-compose-prod.yaml -f docker-compose-android-bundle.yaml up --build --no-start --no-deps app \
&& docker compose -f docker-compose-prod.yaml -f docker-compose-android-bundle.yaml cp app:/srv/app/bundle bundle/temp \
&& mv bundle/temp/bundle/* bundle/android \
&& rm -r bundle/temp

echo "Warning: If a running Docker compose stack has been stopped by this script, you may want to bring it up again now."
