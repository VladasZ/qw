#!/bin/bash
set -eox pipefail

rustup component add clippy

cargo clippy \
    -- \
    \
    -W clippy::all \
    -W clippy::pedantic \
    \
    -A clippy::must-use-candidate \
    -A clippy::missing_errors_doc \
    \
    -D warnings
