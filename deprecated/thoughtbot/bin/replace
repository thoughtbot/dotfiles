#!/bin/sh
#
# Find and replace by a given list of files.
#
# replace foo bar **/*.rb

find_this="$1"
shift
replace_with="$1"
shift

if command -v rg &>/dev/null ; then
  items=$(rg -l --color never "$find_this" "$@")
else
  items=$(ag -l --nocolor "$find_this" "$@")
fi

temp="${TMPDIR:-/tmp}/replace_temp_file.$$"
IFS=$'\n'
for item in $items; do
  sed "s/$find_this/$replace_with/g" "$item" > "$temp" && mv "$temp" "$item"
done
