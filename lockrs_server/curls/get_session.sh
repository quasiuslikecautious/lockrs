session_id="$1"
response=$(curl "http://127.0.0.1:9000/api/v1/sessions/$session_id" \
  --silent \
  --cookie ./lockrs.cookies --cookie-jar ./lockrs.cookies \
  --location)

echo "$response" # | jq --color-output .
