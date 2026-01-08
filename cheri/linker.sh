#!/bin/sh

set -e

cd cheri/doctests

rm -f rust_objects.txt

for arg in "$@"; do
    case "$prev" in
        -o)
            output="$arg"
            ;;
    esac
    case "$arg" in
        *.o|*.rlib)
            echo "$arg" >> rust_objects.txt
            ;;
    esac
    prev="$arg"
done

xmake build

mv build/cheriot/cheriot/release/test_fw $output
