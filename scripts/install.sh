#!/usr/bin/env sh
set -eu
repo="KiddosTech/RCAMP"
version="${RCAMP_VERSION:-}"
if [ -z "$version" ]; then
  version="$(curl -fsSL "https://api.github.com/repos/$repo/releases/latest" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p')"
fi
if [ -z "$version" ]; then echo "Could not determine the latest RCAMP release." >&2; exit 1; fi
install_dir="${RCAMP_BIN_DIR:-$HOME/.local/bin}"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT INT TERM
archive="$tmp_dir/rcamp.tar.gz"
curl -fsSL "https://github.com/$repo/releases/download/$version/rcamp-linux-x86_64.tar.gz" -o "$archive"
mkdir -p "$install_dir"
tar -xzf "$archive" -C "$tmp_dir"
install -m 0755 "$tmp_dir/rcamp" "$install_dir/rcamp"
echo "RCAMP/CLI $version installed at $install_dir/rcamp"
echo "Ensure $install_dir is on PATH, then run: rcamp"
