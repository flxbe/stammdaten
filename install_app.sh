#!/bin/sh

rm -R /Applications/Stammdaten.app
cargo bundle --release
mv ./target/release/bundle/osx/Stammdaten.app /Applications/