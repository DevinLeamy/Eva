#!/bin/bash

cargo build --release > /dev/null 2>&1

cd eva-py

source .env/bin/activate
maturin develop > /dev/null 2>&1

cd ..

python3 "./scripts/$1.py" 
