#!/bin/bash

Log() {
  printf "$(BgBlue "$(Bold " LOG ")") $(Blue "$(Now)") $1\n";
}

Warn() {
  printf "$(BgOrange "$(Bold " WRN ")") $(Yellow "$(Now) $1\n")"
}

Error() {
  printf "$(BgRed "$(Bold " ERR ")") $(Red "$(Now) $1\n")"
}

NewLine() {
  printf "\n"
}
