#!/bin/bash

if [ ! -d ./build ]; then
  mkdir build
fi

for asmfile in *.asm; do
  asar ${asmfile} build/"${asmfile%%.*}.sfc"
done
