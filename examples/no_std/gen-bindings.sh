#!/bin/bash
cargo +nightly run -p gen-bindings --target $(rustc -vV | sed -n 's|host: ||p')