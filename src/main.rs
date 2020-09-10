use std::env;
use std::collections::HashMap;
use lazy_static::lazy_static;

mod parser;
mod solver;

const DIGITS: [u8;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const ROWS: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
const COLS: [u8;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

type Cell = (char, u8);

lazy_static! {
    static ref CELLS: Vec<Cell> = cross(&ROWS, &COLS);

    /// List of units for the sudoku grid: all rows, all columns and all blocks
    static ref UNITSLIST: Vec<Vec<Cell>> = {
        let mut unitlist: Vec<Vec<Cell>> = Vec::new();
        for row in ROWS.iter() {
            let unit = cross(&[*row], &COLS);
            unitlist.push(unit);
        }
        for col in COLS.iter() {
            let unit = cross(&ROWS, &[*col]);
            unitlist.push(unit);
        }
        for block_chars in [['A', 'B', 'C'], ['D', 'E', 'F'], ['G', 'H', 'I']].iter() {
            for block_digits in [[1, 2, 3], [4, 5, 6], [7, 8, 9]].iter() {
                unitlist.push(cross(block_chars, block_digits));
            }
        }
        unitlist
    };

    static ref UNITS: HashMap<Cell, Vec<&'static Vec<Cell>>> = {
        let mut units = HashMap::new();
        for cell in &*CELLS {
            let units_present: Vec<&Vec<Cell>>  = (*UNITSLIST).iter().filter(|v| v.contains(cell)).collect();
            units.insert(*cell, units_present);
        }
        units
    };

    static ref NEIGHBOURS: HashMap<Cell, Vec<Cell>> = {
        let mut neighbours = HashMap::new();
        for cell in &*CELLS {
            let mut cell_neighbours: Vec<Cell> = Vec::new();
            for unit in &*UNITS[cell] {
                cell_neighbours.extend(unit.iter().filter(|c| *c != cell));
            }
            neighbours.insert(*cell, cell_neighbours);
        }
        neighbours
    };
}


pub fn cross(row_slice: &[char], col_slice: &[u8]) -> Vec<Cell> {
    let mut cells: Vec<Cell> = Vec::new();
    for row in row_slice {
        for col in col_slice {
            cells.push((*row, *col));
        }
    }
    cells
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let grid = parser::parse_grid(&arg1);
        let solution_opt = grid.search();
        if let Some(solution) = solution_opt {
            solution.paint();
        } else {
            panic!("No solution");
        }
    } else {
        panic!("Please, provide a grid");
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn cross_test() {
        assert_eq!(vec![('A', 1), ('A', 2)], crate::cross(&['A'], &[1, 2]));
    }
}
