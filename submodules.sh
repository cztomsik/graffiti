#!/bin/sh

set -e

cp .gitmodules .gitmodules.tmp

rm -rf ./libs

git config -f .gitmodules --get-regexp '^submodule\..*\.path$' |
    while read path_key local_path
    do
        url_key=$(echo $path_key | sed 's/\.path/.url/')
        url=$(git config -f .gitmodules --get "$url_key")
        git submodule add $url $local_path
    done

rm .gitmodules
cp .gitmodules.tmp .gitmodules
rm .gitmodules.tmp
