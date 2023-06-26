response=$(curl http://127.0.0.1:8081/api/v1/auth \
  -X POST \
  --silent \
  --location \
  --header 'Authorization: Basic enF1YXNpdXNAZ21haWwuY29tOnBhc3N3b3Jk')

token=$(echo $response | jq -r ".session_token")

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
sh "$script_dir/create_session.sh" "$token"
