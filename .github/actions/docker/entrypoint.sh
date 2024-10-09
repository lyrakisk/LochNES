#!/bin/bash
set -e

SCRIPT=$1

echo -e "Script to run:\n$SCRIPT"

eval "$SCRIPT"
