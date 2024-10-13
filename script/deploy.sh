#!/bin/sh
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# configuration
MOEULE_NAME="rmdsstore"
RELEASE_DIR=/Users/keiichi/home/bin

# deploy
cp -p ${SCRIPT_DIR}/../target/release/$MOEULE_NAME $RELEASE_DIR
