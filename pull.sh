#!/bin/bash
set -e

git pull
docker compose up --build -d
