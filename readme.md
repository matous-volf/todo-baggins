# Todo Baggins

A task management app tailored specifically to my personal GTD workflow.

## Running the web server

### Development

1. Copy `.env.dev` to `.env` and, if desired, edit it.
2. Create a symlink from `docker-compose.yaml` to `docker-compose-dev.yaml`.
3. Run the Docker compose stack with

   ```sh
   docker compose up --build -d
   ```

4. Enter the container shell with

   ```sh
   docker compose exec -it app sh
   ```

5. Inside, start the development server with

   ```sh
   dx serve --locked --addr 0.0.0.0 --port 8000
   ```

6. Once the build is finished, the web app will be accessible at
   <http://localhost:8000>.

### Production

1. Copy `.env.dev` to `.env` and, if desired, edit it.
2. Create a symlink from `docker-compose.yaml` to `docker-compose-prod.yaml`.
3. Run the Docker compose stack with

   ```sh
   docker compose up --build -d
   ```

4. Once the build is finished, the web app will be accessible at the Docker
   container, on port 80.

## Running the Android client

1. Copy `.env.dev` to `.env` and, if desired, edit it. Set the
   `MOBILE_SERVER_URL` to the URL of the server which will be available as a
   backend for the mobile client.
2. Start the build with

   ```sh
   ./scripts/export_android_bundle.sh
   ```

3. Once it is finished, the APK will be present at `bundle/android`.
4. Run the web server as described in a previous section and make it accessible
   from the client running the mobile app at the URL specified in the
   environment variable.
