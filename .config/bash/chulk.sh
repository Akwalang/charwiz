#!/bin/bash

NO_STYLE='\e[0m'

NoStyle() {
  printf "$NO_STYLE"
}

BLACK='\033[0;30m'
RED='\033[0;31m'
GREEN='\033[0;32m'
ORANGE='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
LIGHT_GRAY='\033[0;37m'

DARK_GRAY='\033[1;30m'
LIGHT_RED='\033[1;31m'
LIGHT_GREEN='\033[1;32m'
YELLOW='\033[1;33m'
LIGHT_BLUE='\033[1;34m'
LIGHT_PURPLE='\033[1;35m'
LIGHT_CYAN='\033[1;36m'
WHITE='\033[1;37m'

ColorPrint() {
  if [ $# -eq 1 ]; then
    printf "$1"
  else
    printf "%s" "$1" "${*:2}" "$NO_STYLE"
  fi
}

Black() {
  ColorPrint $BLACK $1
}
Red() {
  ColorPrint $RED $1
}
Green() {
  ColorPrint $GREEN $1
}
Orange() {
  ColorPrint $ORANGE $1
}
Blue() {
  ColorPrint $BLUE $1
}
Purple() {
  ColorPrint $PURPLE $1
}
Cyan() {
  ColorPrint $CYAN $1
}
LightGray() {
  ColorPrint $LIGHT_GRAY $1
}

DarkGray() {
  ColorPrint $DARK_GRAY $1
}
LightRed() {
  ColorPrint $LIGHT_RED $1
}
LightGreen() {
  ColorPrint $LIGHT_GREEN $1
}
Yellow() {
  ColorPrint $YELLOW $1
}
LightBlue() {
  ColorPrint $LIGHT_BLUE $1
}
LightPurple() {
  ColorPrint $LIGHT_PURPLE $1
}
LightCyan() {
  ColorPrint $LIGHT_CYAN $1
}
White() {
  ColorPrint $WHITE $1
}

BG_NC='\e[0m'

BG_BLACK='\e[0;40m'
BG_RED='\e[0;41m'
BG_GREEN='\e[0;42m'
BG_ORANGE='\e[0;43m'
BG_BLUE='\e[0;44m'
BG_PURPLE='\e[0;45m'
BG_CYAN='\e[0;46m'
BG_LIGHT_GRAY='\e[0;47m'

BG_DARK_GRAY='\e[1;40m'
BG_LIGHT_RED='\e[1;41m'
BG_LIGHT_GREEN='\e[1;42m'
BG_YELLOW='\e[1;43m'
BG_LIGHT_BLUE='\e[1;44m'
BG_LIGHT_PURPLE='\e[1;45m'
BG_LIGHT_CYAN='\e[1;46m'
BG_WHITE='\e[1;47m'

BgColorPrint() {
  if [ $# -eq 1 ]; then
    printf "$1"
  else
    printf "%s" "$1" "${*:2}" "$NO_STYLE"
  fi
}

BgBlack() {
  BgColorPrint "$BG_BLACK" "$1"
}
BgRed() {
  BgColorPrint "$BG_RED" "$1"
}
BgGreen() {
  BgColorPrint "$BG_GREEN" "$1"
}
BgOrange() {
  BgColorPrint "$BG_ORANGE" "$1"
}
BgBlue() {
  BgColorPrint "$BG_BLUE" "$1"
}
BgPurple() {
  BgColorPrint "$BG_PURPLE" "$1"
}
BgCyan() {
  BgColorPrint "$BG_CYAN" "$1"
}
BgLightGray() {
  BgColorPrint "$BG_LIGHT_GRAY" "$1"
}

BgDarkGray() {
  BgColorPrint "$BG_DARK_GRAY" "$1"
}
BgLightRed() {
  BgColorPrint "$BG_LIGHT_RED" "$1"
}
BgLightGreen() {
  BgColorPrint "$BG_LIGHT_GREEN" "$1"
}
BgYellow() {
  BgColorPrint "$BG_YELLOW" "$1"
}
BgLightBlue() {
  BgColorPrint "$BG_LIGHT_BLUE" "$1"
}
BgLightPurple() {
  BgColorPrint "$BG_LIGHT_PURPLE" "$1"
}
BgLightCyan() {
  BgColorPrint "$BG_LIGHT_CYAN" "$1"
}
BgWhite() {
  BgColorPrint "$BG_WHITE" "$1"
}

FONT_BOLD='\e[1m'
FONT_DIM='\e[2m'
FONT_ITALIC='\e[3m'
FONT_UNDERLINE='\e[4m'

FontStylePrint() {
  if [ $# -eq 1 ]; then
    printf "$1"
  else
    printf "%s" "$1" "${*:2}" "$NO_STYLE"
  fi
}

Bold() {
  FontStylePrint "$FONT_BOLD" "$1"
}

Dim() {
  FontStylePrint "$FONT_DIM" "$1"
}

Italic() {
  FontStylePrint "$FONT_ITALIC" "$1"
}

Underline() {
  FontStylePrint "$FONT_UNDERLINE" "$1"
}
