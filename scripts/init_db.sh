#!/usr/bin/env bash

set -x
set -eo pipefail

DB_USER="${POSTGRES_USER:=postgres}"

DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

DB_NAME="${POSTGRES_DB:=newsletter}"

DB_PORT="${POSTGRES_PORT:=5431}"

# Launch postgres using Docker
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5431 \
  -d postgres \
  postgres -N 1000
  # ^ Increased maximum number of connections for testing purpose