#!/bin/bash

output=""
for arg in "$@"; do
  case "$prev" in
    -o) output="$arg" ;;
  esac
  prev="$arg"
done

echo '#!/bin/sh' > "$output"
echo 'exit 0' >> "$output"
chmod +x "$output"
