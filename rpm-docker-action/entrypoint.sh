#!/bin/bash

cd /github/workspace
rustup default stable
cargo rpm build
