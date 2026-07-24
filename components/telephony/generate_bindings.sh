#!/usr/bin/env bash
# Regenerate the telephony FFI bindings.
# Intended for reproducibility verification: run this and check `git diff --exit-code`.

set -eu

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ROOT_DIR=$( cd -- "${SCRIPT_DIR}/../.." &> /dev/null && pwd )

if [ -z "${OHOS_SDK_NATIVE:-}" ]; then
    echo "Error: OHOS_SDK_NATIVE environment variable is not set."
    echo "Set it to the 'native' directory of the OpenHarmony SDK."
    exit 1
fi

cd "${ROOT_DIR}/scripts/generator" && ONLY_MODULE=telephony cargo run
