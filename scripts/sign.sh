#!/bin/bash

HMAC_KEY=$(docker exec jw-db psql -U postgres -c "SELECT value FROM secrets WHERE name='hmac_key';" -t -A)

echo -n "$1" | openssl dgst -sha256 -mac hmac -macopt hexkey:$HMAC_KEY -binary | xxd -p -c 64
