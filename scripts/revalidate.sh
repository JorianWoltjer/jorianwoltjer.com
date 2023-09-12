#!/bin/bash

docker exec jw-backend curl -s -X POST http://frontend/api/revalidate -H 'Content-Type: application/json' -d '[{"type":"Custom","slug":"'"$1"'"}]'
