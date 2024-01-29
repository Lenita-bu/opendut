#!/usr/bin/env bash

source "$(dirname "$0")/functions.sh"

ping_all_peers() {
  REQUIRED_SUCCESS="$1"
  IPS=$(netbird status --json | jq -r '.peers.details[].netbirdIp')
  for ip in $IPS
  do
    if [ "$REQUIRED_SUCCESS" == "true" ]; then
      fping -c1 -t500 "$ip" || { echo "$ip did not respond"; exit 1; }
    else
      fping -c1 -t500 "$ip" || { echo "$ip did not respond"; sleep 3; }
    fi
  done
}


wait_for_peers_to_connect

echo "first ping may take multiple seconds"
ping_all_peers "false"
ping_all_peers "false"

set -e  # exit on error
set -x  # print each command

echo "-------------------------------------------------------------------------"
echo "Running ping test"
ping_all_peers "true"

echo "SUCCESS"
exit 0
