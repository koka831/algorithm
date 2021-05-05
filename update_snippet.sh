#!/usr/bin/env bash
set -ue

SNIP_PATH="$HOME/.config/coc/ultisnips/rust/generated.snippets"
SNIP_TYPE="ultisnips"

: "update snippet" && {
    cd lib
    cargo snippet -t "$SNIP_TYPE" >"$SNIP_PATH"
}
