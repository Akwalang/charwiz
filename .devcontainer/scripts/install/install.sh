#!/bin/bash

source "/workspaces/charwiz/.config/bash/incl.sh"
source "/workspaces/charwiz/.devcontainer/scripts/install/incl.sh"

Log "Begin Installation"

install_lang_rust

Log "Installation $(Green "completed")"
