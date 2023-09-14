#!/bin/bash

if [ -z "$1" ]
then
  echo "Usage: $0 <slug>"
  exit 1
fi

docker exec jw-backend curl -s -X POST http://frontend/api/revalidate -H 'Content-Type: application/json' -d '[{"type":"Custom","slug":"'"$1"'"}]'

echo "Revalidated $1"
