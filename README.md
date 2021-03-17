# Experimental Work in Progress

Using Rust with Typescript to produce Web Components that use Web Assembly and HTML5 Audio.

Target was es5, now back es2015 to avoid wp messing up web components. This will change again when I can find the time.

## Caveat

Chromium is/was crap.

`SharedArrayBuffer` objects fail to get garbage collected, causing memory leaks.

* https://bugs.chromium.org/p/chromium/issues/detail?id=921473
* https://bugs.chromium.org/p/chromium/issues/detail?id=877055
* https://bugs.chromium.org/p/chromium/issues/detail?id=935169

## Installation

Install Rust and `wasm-pack`:

* https://rustup.rs/
* https://rustwasm.github.io/wasm-pack/installer/

Then:

  npm install
  npm start