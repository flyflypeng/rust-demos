#!/bin/bash

dir_path=$1
pattern=$2

for file in "$dir_path"/*; do
    if [ -f "$file" ]; then
        sed -i "/^$pattern$/d" "$file"
    fi
done