#!/bin/bash

cd "$(dirname "$0")"

public_ip=$(curl -s https://httpbin.org/ip | jq -r .origin)
export "public_ip=" >> .env
cat .env >> temp.variables