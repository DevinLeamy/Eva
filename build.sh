cd eva-wasm
wasm-pack build --target web
rm -rf ../client/pkg
mv ./pkg ../client
