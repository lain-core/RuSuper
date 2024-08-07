#!/bin/bash

if [ ! -d "build" ]; then
  mkdir build
fi

for file in *.asm; do
  asar $file build/${file%%.*}.sfc
done
