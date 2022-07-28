#!/usr/bin/env bash

set -e

echo "1. Build"

yarn Build

echo
echo "2. Deploy on testnet"

near dev-deploy ./build/debug/simple-counter.wasm

echo
echo "3. dev-deploy done."
echo "account ID : dev-123-456 "

#echo "near call \$CONTRACT init --accountId \$CONTRACT"
echo

exit 0
