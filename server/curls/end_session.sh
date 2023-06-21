session_id="$1"
respone=$(curl "http://127.0.0.1:8081/api/v1/sessions/$session_id" \
  -X DELETE \
  --silent \
  --cookie ./cookies --cookie-jar ./cookies \
  --location)

echo "$response" | jq --color-output .
