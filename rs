#!/bin/bash
set -euo pipefail

update_local
git checkout master
LAST_RELEASE=$(git describe)
NUMBER=$(("${LAST_RELEASE:1}"+1))
git checkout -b release/v$NUMBER
