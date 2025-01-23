#!/bin/bash

source "/workspaces/charwiz/.config/bash/incl.sh"

SETUP_ROOT="/workspaces/charwiz/.devcontainer/scripts/setup"

for dir in "$SETUP_ROOT"/*/; do
  if [ -d "$dir" ]; then
    for script in "$dir"*.sh; do
      if [ -f "$script" ]; then
        Log "Import file: $(Cyan "${script/"/workspaces/charwiz/"/""}")"
        source "$script"
      fi
    done
  fi
done
