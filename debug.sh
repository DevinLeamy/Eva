#!/bin/bash

cargo build --release

cd eva-py

source .env/bin/activate
maturin develop

cd ..

time python3 "./scripts/$1.py"
