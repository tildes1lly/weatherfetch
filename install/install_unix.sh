#!/bin/bash

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
cargo install --git https://github.com/tildes1lly/weatherfetch
weatherfetch