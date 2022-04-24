#!/usr/bin/env bash
set -ue

SNIP_PATH="$HOME/.config/nvim/snippets/rust.generated.snippets"

: "update snippet" && {
    cargo snippet >"$SNIP_PATH"
}
