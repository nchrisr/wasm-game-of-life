//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate rust_wasm_game_of_life;
extern crate wasm_bindgen_test;

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use rust_wasm_game_of_life::{InitialState, Universe, log};

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new(InitialState::ModTwoSeven);
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe{
    let mut universe = Universe::new(InitialState::ModTwoSeven);
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Universe with a small spaceship
    let mut input_universe = input_spaceship();
    log!("Input Universe:\n{}", input_universe);

    // Universe after one tick
    let expected_universe = expected_spaceship();
    log!("Output Universe:\n{}", expected_universe);

    // Call tick and check that the Universe looks as expected
    input_universe.tick();
    log!(" After first tick: Input Universe:\n{}", input_universe);
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
