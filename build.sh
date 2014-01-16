#!/usr/bin/env bash

rustc src/libgame/main.rs --out-dir . -L ./deps/gl-rs/build/x86_64-unknown-linux-gnu/gl -L ./deps/glfw-rs/build/lib --link-args="-lglfw"



