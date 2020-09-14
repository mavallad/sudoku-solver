use crate::{ Cell, GridConstants };
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
    fn init(digits: Vec<u8>) -> DigitOptions {
        DigitOptions { values : digits.clone() }
    }

    fn remove_option(&mut self, val: u8) {
        self.values.retain(|x| *x != val);
    }

    fn contains(&self, value: &u8) -> bool {
        self.values.contains(value)
    }

    fn other_than(&self, value: u8) -> Vec<u8> {
        self.values.iter().cloned().filter(|v| *v != value).collect()
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

#[derive(Clone)]
pub struct Grid<'a> {
    cells_options: HashMap<Cell, DigitOptions>,
    grid_constants: &'a GridConstants,
}
impl<'a> Grid<'a> {
    pub fn init(grid_constants: &GridConstants) -> Grid {
        let mut cells_options = HashMap::new();
        for rcell in &grid_constants.cells {
            cells_options.insert(*rcell, DigitOptions::init(grid_constants.cols.clone()));
        }
        Grid {
            cells_options,
            grid_constants
        }
    }

    /// Using depth-first search and propagate, try all possible values.
    pub fn search(self) -> Option<Grid<'a>> {
        let cell_min_values_opt = self.cells_options
                .iter()
                .filter(|&(_c, digopts)| digopts.values.len() > 1)
                .map(|(c, dopts)| (dopts.values.len(), c))
                .min();

        if let Some((_, cell_min_values)) = cell_min_values_opt {
            let options = &self.cells_options[cell_min_values];
            for option in &options.values {
                let mut cloned_grid = self.clone();
                if cloned_grid.assign(cell_min_values, *option) {
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

    pub fn assign(&mut self, cell: &Cell, value: u8) -> bool {
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
        for c in &self.grid_constants.cells {
            let width = self.cells_options[c].values.len();
            if width > max_width {
                max_width = width;
            }
        }
        max_width += 1;
        let dashes = "-".repeat((max_width + 1) * 3);
        let line = format!("{}+{}+{}", dashes, dashes, dashes);
        for c in &self.grid_constants.cells {
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

    fn eliminate(&mut self, cell: &Cell, value: u8) -> bool {
        if let Occupied(mut options ) = self.cells_options.entry(*cell) {
            let opts = options.get_mut();
            if opts.contains(&value) {
                opts.remove_option(value);
                if opts.no_options() {
                    return false;
                } else if let Some(unique_value) = opts.one_option() {
                    for neighbour in &self.grid_constants.neighbours[cell] {
                        if !self.eliminate(neighbour, unique_value) {
                            return false;
                        }
                    }
                }
                for u in &self.grid_constants.units[&cell] {
                    let mut places_with_value: Vec<&Cell> = Vec::new();
                    for c in u {
                        if self.cells_options[c].contains(&value) {
                            places_with_value.push(c);
                        }
                    }
                    if places_with_value.len() == 0 {
                        return false;
                    } else if places_with_value.len() == 1 {
                        if !self.assign(places_with_value[0], value) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}
