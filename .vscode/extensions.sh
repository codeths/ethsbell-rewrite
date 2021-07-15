#!/bin/bash
# This script installs all recommended extensions.
if which jq; then
	echo "JQ found."
else
	echo "JQ is missing; please install it."
	exit 1
fi
for ex in $(cat $(dirname $0)/extensions.json | jq '.recommendations | join(" ")' | cut -d "\"" -f 2); do
	code --install-extension $ex
done
