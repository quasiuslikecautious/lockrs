user_id="$1"
response=$(curl "http://127.0.0.1:9000/api/v1/users/$user_id" \
  --silent \
  --cookie ./cookies --cookie-jar ./cookies \
  --location)

echo "$response" | jq --color-output .
