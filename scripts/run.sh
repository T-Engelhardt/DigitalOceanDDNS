#!/usr/bin/env bash
set -e
source secret
$1 --api-token $API_TOKEN --domain-name $2