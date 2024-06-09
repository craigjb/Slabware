#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

docker build -t slabware-dev "$SCRIPT_DIR/env"
docker run -it --rm \
  --network=host -e DISPLAY=host.docker.internal:0 \
  -v "$HOME/Projects/fuse-zynq:/home/user/fuse-zynq" \
  -v "$SCRIPT_DIR:/home/user/Slabware" \
  -u user \
  slabware-dev bash
