response=$(curl http://127.0.0.1:9000/api/v1/auth/login \
  -X POST \
  --silent \
  --location \
  --header 'Authorization: Basic emFjaEBxdWFzaXVzLmRldjpwYXNzd29yZA==')

token=$(echo $response | jq -r ".session_token")

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
sh "$script_dir/create_session.sh" "$token"
