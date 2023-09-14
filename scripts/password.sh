#!/bin/bash

password_hash=$(htpasswd -nBC 12 user | cut -d: -f2 | sed 's/$2y/$2a/')

docker exec jw-db psql -U postgres -c "UPDATE secrets SET value='$password_hash' WHERE name='password_hash';"

echo "Password updated."
