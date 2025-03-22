#!bin/sh

cd "client-pages"
pnpm build
cd ..
cargo tauri dev -- -- api-dev
