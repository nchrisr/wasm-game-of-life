/*This line fixes https://github.com/rustwasm/wasm-bindgen/issues/2357 this issue.
  It can also be fixed by adding edition=2018 to cargo.toml under the package declaration,
but I prefer this solution */
extern crate wasm_bindgen;
extern crate js_sys;
extern crate web_sys;

mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math;
use std::collections::HashSet;
use web_sys::console;

#[macro_export]
macro_rules! log {
    ( $ ( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitialState {
    Random = 0,
    SingleShip = 1,
    ModTwoSeven = 2,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize{
        ((row * self.width) + column) as usize
    }

    fn live_neighbour_count( &self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for row_delta in [self.height - 1, 0, 1 ].iter().cloned(){
            for col_delta in [self.width - 1, 0, 1].iter().cloned() {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                }
                let neighbour_row = (row + row_delta) % self.height;
                let neighbour_column = (column + col_delta) % self.width;

                let idx = self.get_index(neighbour_row, neighbour_column);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    fn setup_cells(initial_state: InitialState, width: u32, height: u32) -> Vec<Cell> {

        let size =  width * height;

        let cells = match initial_state {
            InitialState::Random => {
                (0..size).map(|_i| {
                    if Math::random() < 0.5{
                        Cell::Dead
                    }else{
                        Cell::Alive
                    }
                }).collect()
            },

            InitialState::SingleShip => {
                let max_loc = size as f64;
                let random_num = (Math::random() * max_loc).floor() as u32;
                console::log_1(&format!("Random number = {}", random_num).into());

                // Calculate x and y position from random_num
                let tip_x = random_num % width;
                let tip_y = random_num / width;

                console::log_1(&format!("Tip position: ({}, {})", tip_x, tip_y).into());

                // Glider pattern
                let glider_pattern = [
                    (tip_x, tip_y),
                    ((tip_x + 1) % width, (tip_y + 1) % height),
                    ((tip_x + 2) % width, tip_y % height),
                    ((tip_x + 2) % width, (tip_y + 1) % height),
                    ((tip_x + 2) % width, (tip_y + 2) % height),
                ];

                // Use a HashSet for O(1) average time complexity lookups
                let glider_set: HashSet<_> = glider_pattern.iter().cloned().collect();

                (0..size).map(|i| {
                    let x = i % width;
                    let y = i /width;

                    if glider_set.contains(&(x, y)){
                        Cell::Alive
                    }else{
                        Cell::Dead
                    }
                }).collect()
            },

            InitialState::ModTwoSeven => {

                (0..size).map(|i| {
                    if i % 2 == 0 || i % 7 == 0{
                        Cell::Alive
                    }else {
                        Cell::Dead
                    }
                }).collect()
            }
        };

        cells
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(initial_state: InitialState) -> Universe{

        utils::set_panic_hook();

        let width : u32 = 64;
        let height : u32 = 64;

        let cells = Universe::setup_cells(initial_state, width, height);

        Universe{
            width,
            height,
            cells,
        }
    }

    pub fn reset(&mut self, initial_state: InitialState) {
        let new_cells = Universe::setup_cells(initial_state, self.width, self.height);
        self.cells = new_cells;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let cell_idx = self.get_index(row, col);
        self.cells[cell_idx].toggle();
    }
    
    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbours
                );

                let next_cell = match (cell, live_neighbours) {
                    // Rule 1: Any live cell with fewer than two live neighbours dies (Underpopulation).
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours lives on to the next gen.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    //Rule 3 Any live cell with more than three live neighbours lives (Overpopulation).
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    //Rule 4: Any dead cell with exactly three live neighbours comes alive (Reproduction).
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                log!("    it becomes {:?}", next_cell);


                next[idx] = next_cell;
            }
        }
        self.cells = next
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Universe{

    // Get the dead and alive values for the entire Universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    // Set cells to be alive in a Universe by passing the row and column of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]){
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

    /*
    Set the width of the Universe, and reset all cells to dead state.
    */
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /*
    Set the height of the Universe, and reset all cells to the dead state.
    */
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
}


impl fmt::Display for Universe{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {'◻'} else {'◼'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
