#!/bin/bash

set -x

cargo build --release

ln -s $(pwd)/target/release/chatgpt /usr/local/bin/chatgpt