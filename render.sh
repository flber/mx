#!/bin/bash

file=$1
extension="${file##*.}"

when=$(date -r $file "+%d%m%C")
sed -i "s/{{current_date}}/$when/g" "$file"
touch mod.$extension
sed "s/{{date}}/$when/g" $file > mod.$extension
sed -i "s/{{feed}}//g" mod.$extension

cmark -t html mod.$extension
rm mod.$extension
