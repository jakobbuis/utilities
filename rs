#!/bin/bash
set -euo pipefail

update_local
git checkout master
LAST_RELEASE=$(cut -d '-' -f1 <<< $(git describe HEAD))
NUMBER=$(("${LAST_RELEASE:1}"+1))
git checkout -b release/v$NUMBER
