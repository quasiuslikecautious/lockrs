user_id="$1"
password="$2"

response=$(curl "http://127.0.0.1:9000/api/v1/users/$user_id" \
  -X PUT \
  --silent \
  --cookie ./cookies --cookie-jar ./cookies \
  --location \
  --json '{"password": "'"$password"'"}')

echo "$response" | jq --color-output .
