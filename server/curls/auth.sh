response=$(curl http://127.0.0.1:8081/api/v1/auth \
  -X POST \
  --silent \
  --location \
  --header 'Authorization: Basic enF1YXNpdXNAZ21haWwuY29tOnBhc3N3b3Jk')

echo $response | jq --color-output .
