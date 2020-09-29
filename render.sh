#!/bin/bash

file=$1
extension="${file##*.}"

when=$(date -r $file "+%d%m%C")
touch mod.$extension
sed "s/{{date}}/$when/g" $file > mod.$extension

cmark -t html mod.$extension
rm mod.$extension
