#!/usr/bin/env bash

set -e

wasm-pack build --target web --out-name wasm --out-dir ./static

miniserve ./static --index index.html
