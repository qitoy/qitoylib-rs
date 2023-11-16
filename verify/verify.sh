#!/bin/bash -eu

dir=`dirname $0`

cat "$dir/verify.json" |
  jq -r 'walk(if type == "object" then to_entries | map([.key, .value]) else . end) | .[] | [.[0]] + .[1][] | @tsv' |
  xargs printf "$dir/verify/%s/%s %s " |
  xargs -P4 -n2 oj download --system --format %n/%e --directory


