use crate::Cell;
use crate::{DIGITS, CELLS, UNITS, NEIGHBOURS};
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;

pub enum SudokuResult {
    NoSolution,
    Solvable
}

#[derive(Debug)]
pub struct DigitOptions {
    values: Vec<u8>
}
impl DigitOptions {
    fn init() -> DigitOptions {
        DigitOptions { values : DIGITS.to_vec() }
    }

    fn of_value(val: u8) -> DigitOptions {
        DigitOptions { values : vec![val] }
    }

    fn remove_option(&mut self, val: u8) {
        self.values.retain(|x| *x != val);
    }

    fn contains(&self, value: &u8) -> bool {
        self.values.contains(value)
    }

    fn one_option(&self) -> Option<u8> {
        if self.values.len() == 1 {
            Some(self.values[0])
        } else {
            None
        }
    }

    fn no_options(&self) -> bool {
        self.values.len() == 0
    }
}

#[derive(Debug)]
pub struct Grid {
    cells_options: HashMap<Cell, DigitOptions>
}
impl Grid {
    pub fn init() -> Grid {
        let mut cells_options = HashMap::new();
        for rcell in &*CELLS {
            cells_options.insert(*rcell, DigitOptions::init());
        }
        Grid {
            cells_options
        }
    }

    pub fn assign(&mut self, cell: Cell, value: u8) {
        &self.cells_options.insert(cell, DigitOptions::of_value(value));
        for neighbour in &NEIGHBOURS[&cell] {
            self.eliminate(*neighbour, value);
        }
    }

    pub fn values(&self, cell: &Cell) -> &Vec<u8> {
        &self.cells_options[cell].values
    }

    fn eliminate(&mut self, cell: Cell, value: u8) -> bool {
        if let Occupied(mut options ) = self.cells_options.entry(cell) {
            let opts = options.get_mut();
            if opts.contains(&value) {
                opts.remove_option(value);
                if opts.no_options() {
                    return false;
                } else if let Some(unique_value) = opts.one_option() {
                    for neighbour in &NEIGHBOURS[&cell] {
                        self.eliminate(*neighbour, unique_value);
                    }
                }
            }
            for u in &*UNITS[&cell] {

            }
        }
        true
    }
}

fn solve() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_test() {
        assert_eq!(vec![('A', 1), ('A', 2)], crate::cross(&['A'], &[1, 2]));
    }
}
