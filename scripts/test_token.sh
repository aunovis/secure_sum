#!/bin/bash

cd "$(git rev-parse --show-toplevel)"

if [[ -f .env ]]; then
  source .env
fi

if [ -z $GITHUB_TOKEN ]; then
    echo "No variable called GITHUB_TOKEN was found in the environment."
    exit 1
fi

HTTP_BODY_AND_CODE="$(
  curl -sS --fail-with-body \
       -H "Authorization: Bearer ${GITHUB_TOKEN}" \
       -H "Accept: application/vnd.github+json" \
       -H "User-Agent: token-validator-script" \
       -w "\n%{http_code}" \
       https://api.github.com/rate_limit \
  || true
)"

echo "Using GITHUB_TOKEN token ***$(echo $GITHUB_TOKEN | tail -c 5)."

HTTP_CODE="$(echo "$HTTP_BODY_AND_CODE" | tail -n1)"

if (($HTTP_CODE > 399)); then
    echo "❌Error: Received HTTP return code $HTTP_CODE."
    exit 1
else
    echo "✅All good: Received HTTP return code $HTTP_CODE."
    echo "The Token is valid."
fi
