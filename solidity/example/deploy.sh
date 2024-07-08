#!/bin/bash

# This script will deploy the specify contract to Sepolia

if [ ! $# -eq 2 ]; then
  echo "Invalid number of arguments. Required exactly 2 arguments"
  exit
fi

# First argument
PRIVATE_KEY=$1

# Second argument
SOURCE=$2

forge create \
  --constructor-args 0 \
  --rpc-url https://ethereum-sepolia-rpc.publicnode.com \
  --private-key $PRIVATE_KEY \
  $SOURCE


