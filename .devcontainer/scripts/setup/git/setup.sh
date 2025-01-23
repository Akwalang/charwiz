#!/bin/bash

setup_git() {
  local NAME="git"

  local HOOK_SOURCE=".devcontainer/scripts/setup/git/hooks"
  local HOOK_DESTINATION=".git/hooks"

  for file in "$HOOK_SOURCE"/*; do
    local name=$(basename $file)

    if [ -f "$file" ]; then
      cp "$file" "$HOOK_DESTINATION/$name"
      Log "$(Purple "$NAME"): setup hook $(Cyan "$name")"
    fi
  done
}
