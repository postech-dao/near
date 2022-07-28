#!/usr/bin/env bash

set -e

echo "1. Build"

yarn Build

echo
echo "2. Deploy on mainnet"

export NEAR_ENV=mainnet

near login

near deploy <accountId> ./build/debug/simple-counter.wasm

echo
echo "3. deploy done."
echo 

exit 0
