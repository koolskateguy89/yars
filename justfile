#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.

# https://github.com/casey/just

# List available recipes
default:
  @just --list --unsorted

simple:
  cargo run --example simple

form:
  cargo run --example form
