#!/bin/sh
#
# Find and replace by a given list of files.
#
# replace foo bar **/*.rb

find_this="$1"
shift
replace_with="$1"
shift

items=$(ag -l --nocolor "$find_this" "$@")
temp="${TMPDIR:-/tmp}/replace_temp_file.$$"
IFS=$'\n'
for item in $items; do
  sed "s/$find_this/$replace_with/g" "$item" > "$temp" && mv "$temp" "$item"
done
