# blossom-wasm

## Setup
```
cargo install wasm-pack
```

## Build
```
# For stable release
wasm-pack build
# For nightly release
rustup run nightly wasm-pack build
```

## Test
```
# For stable release
wasm-pack test --headless --firefox
# For nightly release
rustup run nightly wasm-pack test --headless --firefox
```

## Run
```
# import the module in your javascript e.g.
import { Blossom } from '/path/to/blossom/pkg/blossom_wasm';

