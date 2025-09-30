#!/usr/bin/env sh
set -e
HOSTPORT="$1"; shift
HOST=$(echo "$HOSTPORT" | cut -d: -f1)
PORT=$(echo "$HOSTPORT" | cut -d: -f2)
until nc -z "$HOST" "$PORT"; do
  echo "Waiting for $HOST:$PORT..."
  sleep 1
done
[ "$#" -gt 0 ] && exec "$@"
