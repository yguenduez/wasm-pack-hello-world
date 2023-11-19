# Wasm-Pack Hello World

This is a small example shown in the IoT Basel Meetup on
November 21st.

This shows how you can create a webassembly (wasm) in rust, generate the needed glue code and call the exported rust
functions
in a nodejs project.

## Prerequisites

- Node with npm: see [here](https://nodejs.org/en/download).
- rust: see [here](https://www.rust-lang.org/tools/install).

After having Rust installed, you can install `wasm-pack` with `cargo`, rust's package manager and main tool.

`cargo install wasm-pack`

## Building

To build the wasm and its glue code, you can just call:

```sh
wasm-pack build --target nodejs
```

This will build the webassembly and will generate a ready to use node package inside the `pkg` dir.

(For completeness, already done for you) To install this local node package: `npm install ./pkg`

## Running

As the package is already included in the `index.js` file - You can execute the .js script with `npm run dev`.