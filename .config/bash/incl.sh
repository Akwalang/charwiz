#!/bin/bash

DIR="${BASH_SOURCE%/*}"
if [[ ! -d "$DIR" ]]; then DIR="$PWD"; fi

. "$DIR/variables.sh"
. "$DIR/chulk.sh"
. "$DIR/now.sh"
. "$DIR/logger.sh"
