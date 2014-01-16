#!/usr/bin/env bash


pushd deps/glfw-rs
mkdir build
cd build
cmake ..
make
popd

pushd deps/gl-rs
rustpkg build --opt-level=3 gl
popd
