#!/bin/sh

function kill_localhost3000() {
  pid=$(lsof -t -i :3000)
  if [ -n "$pid" ]; then
    echo "Port 3000 is in use. Killing the process..."
    kill -9 $pid
  fi
}

trap kill_localhost3000 EXIT

cd "client-pages"
pnpm dev &
cd ..
cargo tauri dev -- -- client-pages-dev api-dev
