[![Build Status](https://travis-ci.com/ergoplatform/sigma-rust.svg?branch=develop)](https://travis-ci.com/ergoplatform/sigma-rust)
[![Coverage Status](https://coveralls.io/repos/github/ergoplatform/sigma-rust/badge.svg)](https://coveralls.io/github/ergoplatform/sigma-rust)

Rust implementation of [ErgoScript](https://github.com/ScorexFoundation/sigmastate-interpreter) cryptocurrency scripting language. 

## Crates
[ergo-lib](https://github.com/ergoplatform/sigma-rust/tree/develop/ergo-lib)[![Latest Version](https://img.shields.io/crates/v/ergo-lib.svg)](https://crates.io/crates/ergo-lib) [![Documentation](https://docs.rs/ergo-lib/badge.svg)](https://docs.rs/crate/ergo-lib)
ErgoTree AST, interpreter, chain types (transactions, boxes, etc.), JSON serialization, tx creation and signing.

Bindings:
- JS/TS(WASM) [ergo-lib-wasm](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-wasm) [![Latest Version](https://img.shields.io/crates/v/ergo-lib-wasm.svg)](https://crates.io/crates/ergo-lib-wasm) [![Documentation](https://docs.rs/ergo-lib-wasm/badge.svg)](https://docs.rs/crate/ergo-lib-wasm)
- iOS [ergo-lib-ios](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-ios)
- Android(JNI) [ergo-lib-android](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-android)
- C [ergo-lib-c](https://github.com/ergoplatform/sigma-rust/tree/develop/bindings/ergo-lib-c)

## Contributing
Contributions are very welcome! Checkout issues labeled ["help wanted" and "good first issue"](https://github.com/ergoplatform/sigma-rust/labels/help%20wanted)
We check code formatting and linter(`clippy`) as part of the CI process. It's better to set up your editor to run `rustfmt` on file save.
 Feel free to disable specific linter rules in the source code when appropriate.
