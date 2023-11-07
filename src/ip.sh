#!/bin/bash

ip=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | awk 'NR==1{print $2}')
echo "API_URL=http://${ip}:5000" > src/.envFrontend