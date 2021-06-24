#!/bin/bash

cd client && wasm-pack build --target web && cd - && cargo run server