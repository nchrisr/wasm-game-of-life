//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate rust_wasm_game_of_life;
use rust_wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
