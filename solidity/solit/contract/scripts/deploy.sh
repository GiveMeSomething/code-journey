#!/bin/bash

SEPOLIA_RPC_URL="https://ethereum-sepolia-rpc.publicnode.com"

# Check if forge is install
if ! command -v forge >/dev/null 2>&1
then
	echo "forge could not be found. Aborting deployment..."
	exit 1
else
	echo "forge found"
	echo
fi

POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
	case $1 in
		-rpc|--rpc-url)
			RPC_URL="$2"
			shift
			shift
			;;
		-pk|--private-key)
			PRIVATE_KEY="$2"
			shift
			shift
			;;
		-f|--file-path)
			CONTRACT_PATH="$2"
			shift
			shift
			;;
		-*|--*)
			echo "Unknown option $1"
			exit 1
	esac
done

set -- "${POSITIONAL_ARGS[@]}" # restore positional parameters

# Check input
if [[ -z "${PRIVATE_KEY}" ]]; then
	echo "Cannot deploy contract without a private key. Add a private key with -pk or --private-key"
	exit 1
fi

if [[ -z "${CONTRACT_PATH}" ]]; then
	echo "You need to specify a contract to deploy. Specify one with -f or --file-path"
	exit 1
fi

echo "Input received"
if [[ -z "${RPC_URL}" ]]; then
	echo "Empty RPC URL detected"
	echo "Defaulting to Sepolia Testnet RPC: ${SEPOLIA_RPC_URL}"
	RPC_URL="${SEPOLIA_RPC_URL}"
else
	echo "RPC_URL: ${RPC_URL}"
fi

echo "Contract path: ${CONTRACT_PATH}"
echo

# Confirmation
read -p "Deploy? (Y/N)" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
	echo "Contract deployment denied. Aborting..."
	exit 1
fi

read -p "Constructor arguments: " -r
echo

CONTRACT_PATH_PATHNAME=$(dirname "${CONTRACT_PATH}")
CONTRACT_PATH_FILENAME=$(basename "${CONTRACT_PATH}")

forge create \
	--rpc-url "${RPC_URL}" \
	--private-key ${PRIVATE_KEY} \
	--root "${CONTRACT_PATH_PATHNAME}" \
	--use 0.8.26 \
	"${CONTRACT_PATH_FILENAME}" \
	--constructor-args $REPLY


