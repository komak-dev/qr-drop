# QRDrop

QRDrop is a simple file sharing service that allows you to share files with others by scanning a QR code. It is built using Tauri and Next.js.


## How to use `.sh` files

### `init.sh`
This script will install all the dependencies for the project.
You should run this script after cloning the repository.

### `dev.sh`
This script will start the development server at `http://{your_ip}:8080`.
`/client-pages/out` will be served as the client pages.

### `dev_client_pages.sh`
This script will start the development server for api at `http://{your_ip}:3000`, and the client pages at `http://{your_ip}:8080`.
You can develop the Next.js for client pages with hot reloading.

### `dev_build.sh`
This script will build Next.js at `/client-pages` directory before starting the development server at `http://{your_ip}:8080`.

### `build.sh`
This script will build Next.js at `/client-pages` directory and then build the Tauri app.
