use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

mod parser;
mod solver;

// const DIGITS: [u8;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
// const ROWS: [char; 9] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
// const COLS: [u8;9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

type Cell = (char, u8);
#[derive(Clone)]
pub struct GridConstants {
    cols: Vec<u8>,
    cells: Vec<Cell>,
    /// List of units for the sudoku grid: all rows, all columns and all blocks
    unitlist: Vec<Vec<Cell>>,
    /// For each cell, the list of units associated
    units: HashMap<Cell, Vec<Vec<Cell>>>,
    /// For each cell, the list of cells that relate to it (same row, same column or same block)
    neighbours: HashMap<Cell, HashSet<Cell>>,
}

impl GridConstants {
    pub fn init() -> GridConstants {
        let rows = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I'];
        let cols = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cells = Self::cross(&rows, &cols);
        let mut unitlist = Vec::<Vec<Cell>>::with_capacity(27);
        for row in &rows {
            unitlist.push(Self::cross(&[*row], &cols));
        }
        for col in &cols {
            unitlist.push(Self::cross(&rows, &[*col]));
        }
        for block_chars in [['A', 'B', 'C'], ['D', 'E', 'F'], ['G', 'H', 'I']].iter() {
            for block_digits in [[1, 2, 3], [4, 5, 6], [7, 8, 9]].iter() {
                unitlist.push(Self::cross(block_chars, block_digits));
            }
        }
        let mut units: HashMap<Cell, Vec<Vec<Cell>>> = HashMap::with_capacity(81);
        for cell in &cells {
            let units_present: Vec<Vec<Cell>>  = unitlist.iter().cloned().filter(|v| v.contains(cell)).collect();
            units.insert(*cell, units_present);
        }
        let mut neighbours = HashMap::with_capacity(81);
        for cell in &cells {
            let mut cell_neighbours: HashSet<Cell> = HashSet::with_capacity(24);
            for unit in &units[cell] {
                cell_neighbours.extend(unit.iter().filter(|c| *c != cell));
            }
            neighbours.insert(*cell, cell_neighbours);
        }
        GridConstants { cols, cells, unitlist, units, neighbours }
    }

    fn cross(row_slice: &[char], col_slice: &[u8]) -> Vec<Cell> {
        let mut cells: Vec<Cell> = Vec::new();
        for row in row_slice {
            for col in col_slice {
                cells.push((*row, *col));
            }
        }
        cells
    }    
}

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let grid_constants = GridConstants::init();
        let result = parser::parse_grid(&arg1, &grid_constants);
        let grid = match result {
            Ok(grid) => grid,
            Err(message) => {
                eprintln!("{}", message);
                return;
            }
        };
        let solution_opt = grid.search();
        if let Some(solution) = solution_opt {
            solution.paint();
        } else {
            eprintln!("No solution");
        }
    } else {
        eprintln!("Please, provide a grid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_test() {
        assert_eq!(vec![('A', 1), ('A', 2)], GridConstants::cross(&['A'], &[1, 2]));
    }

    #[test]
    fn grid_constants_test() {
        let grid_constants = GridConstants::init();
        assert_eq!(81, grid_constants.cells.len());
        assert_eq!(27, grid_constants.unitlist.len());
        assert!(grid_constants.cells.iter().all(|c| grid_constants.units[c].len() == 3));
        // grid_constants.cells.iter().for_each(|c| println!("{}", grid_constants.neighbours[c].len()));
        assert!(grid_constants.cells.iter().all(|c| grid_constants.neighbours[c].len() == 20));
        assert_eq!(vec![vec![('C', 1 as u8), ('C', 2), ('C', 3), ('C', 4), ('C', 5), ('C', 6), ('C', 7), ('C', 8), ('C', 9)],
                        vec![('A', 2 as u8), ('B', 2), ('C', 2), ('D', 2), ('E', 2), ('F', 2), ('G', 2), ('H', 2), ('I', 2)],
                        vec![('A', 1 as u8), ('A', 2), ('A', 3), ('B', 1), ('B', 2), ('B', 3), ('C', 1), ('C', 2), ('C', 3)]],
                   grid_constants.units[&('C', 2)]);
        assert_eq!(vec![('A', 2), ('B', 2), ('D', 2), ('E', 2), ('F', 2), ('G', 2), ('H', 2), ('I', 2),
                        ('C', 1), ('C', 3), ('C', 4), ('C', 5), ('C', 6), ('C', 7), ('C', 8), ('C', 9),
                        ('A', 1), ('A', 3), ('B', 1), ('B', 3)].into_iter().collect::<HashSet<Cell>>(),
                   grid_constants.neighbours[&('C', 2)]);
    }
}


// def test():
//     "A set of unit tests."
//     assert len(squares) == 81
//     assert len(unitlist) == 27
//     assert all(len(units[s]) == 3 for s in squares)
//     assert all(len(peers[s]) == 20 for s in squares)
//     assert units['C2'] == [['A2', 'B2', 'C2', 'D2', 'E2', 'F2', 'G2', 'H2', 'I2'],
//                            ['C1', 'C2', 'C3', 'C4', 'C5', 'C6', 'C7', 'C8', 'C9'],
//                            ['A1', 'A2', 'A3', 'B1', 'B2', 'B3', 'C1', 'C2', 'C3']]
//     assert peers['C2'] == set(['A2', 'B2', 'D2', 'E2', 'F2', 'G2', 'H2', 'I2',
//                                'C1', 'C3', 'C4', 'C5', 'C6', 'C7', 'C8', 'C9',
//                                'A1', 'A3', 'B1', 'B3'])
//     print 'All tests pass.'