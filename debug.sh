#!/bin/bash

cargo build --release

cd wall-e-py

source .env/bin/activate
maturin develop

cd ..

time python3 "./scripts/$1.py"
