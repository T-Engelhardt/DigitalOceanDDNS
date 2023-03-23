#!/usr/bin/env bash
set -e
source secret
cargo run -- --api-token $API_TOKEN --domain-name $1