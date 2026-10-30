#!/bin/sh
# Packs one plugin as out/<plugin>-<version>-<platform>.vcw for the release assets.
# Usage: tools/pack-plugin.sh <plugin folder name> [rust|python]; SMITH_PLATFORM is set by `smith release assets`.
# vcw is private: $VIDEOCRAW is the VideoCraw checkout the pipeline puts beside this repository.
set -e
app="$1"; kind="${2:-rust}"
cargo build --release --manifest-path "$VIDEOCRAW/Cargo.toml" -p videocraw-vcw
vcw="$VIDEOCRAW/dist/cargo/release/vcw"
[ -f "$vcw.exe" ] && vcw="$vcw.exe"
mkdir -p out
if [ "$kind" = python ]; then
  "$vcw" pack "apps/$app" --platform any --out out
else
  crate="videoplugins-$app"
  cargo build --release -p "$crate"
  program="dist/cargo/release/$crate"
  [ -f "$program.exe" ] && program="$program.exe"
  "$vcw" pack "apps/$app" --program "$program" --out out
fi
