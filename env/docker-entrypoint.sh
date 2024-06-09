#!/bin/bash
source /tools/Xilinx/Vivado/2022.2/settings64.sh
echo "set -o vi" >> /home/user/.bashrc
exec "$@"
