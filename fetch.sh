# Fetches the input for a given day and saves it to inputs/dayXX.in
# Usage: ./fetch.sh XX
#
# To get your session cookie, follow the instructions at:
# https://github.com/wimglenn/advent-of-code-wim/issues/1
#
# Then set the AOC_SESSION environment variable to the output of that command.
# Save this in ~/.config/aoc/token to avoid having to do this every time.
#
# Acknowledgements:
# This script was taken from https://github.com/AxlLind/AdventOfCode2022/blob/main/fetch.sh

#!/bin/bash
set -euo pipefail
SCRIPT_DIR=$(realpath "$(dirname "$0")")
AOC_SESSION=$(<~/.config/aoc/token)

if [[ $# != 1 ]]; then
  echo Please provide a day number.
  echo usage: "$0" DAY
  exit 1
fi

if [[ ! "$1" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  echo Not a valid day: "$1"
  exit 1
fi

if [[ -z "${AOC_SESSION-""}" ]]; then
  echo \$AOC_SESSION not set
  exit 1
fi

mkdir -p "$SCRIPT_DIR/inputs"

curl -s "https://adventofcode.com/2023/day/${1#0}/input" \
    --cookie "session=$AOC_SESSION" \
    -A "Bash script at $(git remote -v | awk 'NR==1{print $2}')" \
    | tee "$SCRIPT_DIR/inputs/day$1.in"


