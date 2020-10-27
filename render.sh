#!/bin/bash

# variable
file=$1
extension="${file##*.}"
when=$(date -r $file "+%d%m%C")

# edit actual file here
if grep -q '{{current_date}}' $file; then
	sed -i "s/{{current_date}}/$when/g" $file
fi

touch mod.$extension

# edit md for output here
sed "s/{{date}}/$when/g" $file > mod.$extension
sed -i "s/{{feed}}//g" mod.$extension
sed -i "s/({{variables[\s\S]+}})//g" mod.$extension

cmark -t html mod.$extension
rm mod.$extension
