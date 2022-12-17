#!/bin/bash
set -e

/app/wait-for-it.sh database:5432 -q

bash -c "$@"
