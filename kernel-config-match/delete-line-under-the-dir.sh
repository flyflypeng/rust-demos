#!/bin/bash

dir_path="/path/to/your/directory"
pattern="given_string"

for file in "$dir_path"/*; do
    if [ -f "$file" ]; then
        sed -i "/^$pattern$/d" "$file"
    fi
done