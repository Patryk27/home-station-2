#!/bin/bash

shopt -s nullglob

for file in *.svg; do
    inkscape -z -e "${file%%.*}.png" -w 512 -h 512 "$file"
done