#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

csrutil status
# Gatekeeper
spctl --status
