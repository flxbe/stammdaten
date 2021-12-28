#!/bin/sh


cargo bundle --release
rm -R /Applications/Stammdaten.app
mv ./target/release/bundle/osx/Stammdaten.app /Applications/