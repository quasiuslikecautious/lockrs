session_token="$1"
response=$(curl http://127.0.0.1:9000/api/v1/sessions \
  -X POST \
  --silent \
  --location \
  --cookie ./lockrs.cookies --cookie-jar ./lockrs.cookies \
  --header 'Authorization: Bearer '"$session_token"'')

echo "$response" | jq --color-output .
