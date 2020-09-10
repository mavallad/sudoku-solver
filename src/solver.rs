use crate::Cell;
use crate::{DIGITS, CELLS, UNITS, NEIGHBOURS};
use std::collections::HashMap;
use std::collections::hash_map::Entry::Occupied;

// pub enum SudokuResult {
//     NoSolution,
//     Solvable
// }

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    /// Using depth-first search and propagation, try all possible values.
    pub fn search(self) -> Option<Grid> {
        let mut cell_min_values_opt = None;
        let mut min_values_len = 10;
        for (cell, options) in &self.cells_options {
            let options_len = options.values.len();
            if options_len > 1 && options_len < min_values_len {
                cell_min_values_opt = Some(cell);
                min_values_len = options_len;
            }
        }
        if let Some(cell_min_values) = cell_min_values_opt {
            let options = &self.cells_options[cell_min_values];
            for option in &options.values {
                let mut cloned_grid = self.clone();
                if cloned_grid.assign(*cell_min_values, *option) {
                    if let Some(solution) = cloned_grid.search() {
                        return Some(solution);
                    }
                }
            }
            None
        } else {
            Some(self)
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
                if c.0 == 'C' || c.0 == 'F' {
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
                        if !self.eliminate(*neighbour, unique_value) {
                            return false;
                        }
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
