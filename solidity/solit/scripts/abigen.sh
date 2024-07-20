#!/bin/bash

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
		-f|--file-path)
			CONTRACT_PATH="$2"
			shift
			shift
			;;
		-o|--output)
			OUTPUT_PATH="$2"
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
if [[ -z "${CONTRACT_PATH}" ]]; then
	echo "You need to specify a contract to deploy. Specify one with -f or --file-path"
	exit 1
fi

if [[ -z "${OUTPUT_PATH}" ]]; then
	echo "Output path not found"
	echo "Defaulting to current folder"
	OUTPUT_PATH="$(pwd)"
fi

echo "Contract path: ${CONTRACT_PATH}"
echo "Output path: ${OUTPUT_PATH}"
echo

# Generation
mkdir --parents "${OUTPUT_PATH}"
forge inspect "${CONTRACT_PATH}" abi > "${OUTPUT_PATH}"



