#!/bin/sh

HEADER='\033[1;92m[Script]\033[0m'
HOSTS_FILE="/etc/hosts"
SELF_HOSTS_FILE="$(cat $(dirname "$0")/hosts)"
HOST="127.0.0.1"

echo -e "$HEADER Adding local domain to $HOSTS_FILE file"
for line in $SELF_HOSTS_FILE
do
    if sudo cat /etc/hosts |
        grep -qF "$line"
    then
        echo "Host ($line) already configured, skipping !"
    else
        echo "Writing host ($line) in $HOSTS_FILE"
        echo -e "$HOST\t$line" | sudo tee -a /etc/hosts
    fi
done
