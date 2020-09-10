use crate::Cell;
use crate::{DIGITS, CELLS, UNITS, NEIGHBOURS};
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;

// pub enum SudokuResult {
//     NoSolution,
//     Solvable
// }

#[derive(Debug)]
pub struct DigitOptions {
    values: Vec<u8>
}
impl DigitOptions {
    fn init() -> DigitOptions {
        DigitOptions { values : DIGITS.to_vec() }
    }

    fn remove_option(&mut self, val: u8) {
        self.values.retain(|x| *x != val);
    }

    fn contains(&self, value: &u8) -> bool {
        self.values.contains(value)
    }

    fn other_than(&self, value: u8) -> Vec<u8> {
        self.values.iter().filter(|v| **v != value).map(|v| *v).collect()
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

    pub fn assign(&mut self, cell: Cell, value: u8) -> bool {
        let options_to_remove = self.cells_options[&cell].other_than(value);
        for other_value in options_to_remove {
            if !self.eliminate(cell, other_value) {
                return false;
            }
        }
        true
    }

    /// Display these values as a 2-D grid.
    pub fn paint(&self) {
        let mut max_width = 0;
        for c in &*CELLS {
            let width = self.cells_options[c].values.len();
            if width > max_width {
                max_width = width;
            }
        }
        max_width += 1;
        let dashes = "-".repeat((max_width + 1) * 3);
        let line = format!("{}+{}+{}", dashes, dashes, dashes);
        for c in &*CELLS {
            print!("{:?}", &self.cells_options[c].values);
            if c.1 == 3 || c.1 == 6 {
                print!("|");
            }
            if c.1 == 9 {
                println!();
                if c.0 == 'B' || c.0 == 'E' {
                    println!("{}", line);
                }
            }
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
                let mut places_with_value: Vec<&Cell> = Vec::new();
                for c in *u {
                    if self.cells_options[c].contains(&value) {
                        places_with_value.push(c);
                    }
                }
                if places_with_value.len() == 0 {
                    return false;
                } else if places_with_value.len() == 1 {
                    if !self.assign(*places_with_value[0], value) {
                        return false;
                    }
                }
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
