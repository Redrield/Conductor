#!/bin/bash

cd /github/workspace
rustup default stable
ls
cargo rpm build
