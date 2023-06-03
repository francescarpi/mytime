#!/bin/bash

# Build final releases

if [ ! -d "releases" ];
then
  mkdir releases
fi

cargo build --release
VERSION=$(./target/release/mytime -V | awk '{ print $2}')
DEST="../../releases/mytime.mac-arm.${VERSION}.zip"

cd target/release && zip -r $DEST mytime

echo "Done!"
