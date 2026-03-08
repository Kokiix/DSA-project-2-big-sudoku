### march 8th sun

- follow [mozilla WASM guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm#rust_and_webassembly_use_cases) to setup
- immediate question: what type do I return? naive 2D vec doesn't work because it's missing some conversion into JS trait
  - I need to return both a completed grid, and the tiles to remove in order to make it a proper puzzle
  - looks like returning my own Game type, bound with bindgen attributes, is the best option
