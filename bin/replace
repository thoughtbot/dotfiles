#!/bin/sh
#
# Find and replace by a given list of files.
#
# replace foo bar **/*.rb

find_this=$1
shift
replace_with=$1
shift

ag -l $find_this $* | xargs sed -i '' "s/$find_this/$replace_with/g"
