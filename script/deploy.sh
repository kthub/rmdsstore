#!/bin/sh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# configuration
MOEULE_NAME="rmdsstore"
RELEASE_DIR=/usr/local/bin

# deploy
cp -p ${SCRIPT_DIR}/../target/release/$MOEULE_NAME $RELEASE_DIR
