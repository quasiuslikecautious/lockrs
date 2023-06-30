session_id="$1"
response=$(curl "http://127.0.0.1:8081/api/v1/sessions/$session_id" \
  --silent \
  --cookie ./cookies --cookie-jar ./cookies \
  --location)

echo "$response" # | jq --color-output .
