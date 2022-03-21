./target/release/node-template benchmark \
--chain dev \
--execution wasm \
--wasm-execution compiled \
--pallet pallet_kitties \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--json-file=raw.json \
--output ./pallets/kitties/src/weights_new.rs
