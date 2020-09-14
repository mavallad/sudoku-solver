use crate::solver::Grid;
use crate::GridConstants;

pub fn parse_grid<'a>(grid_str: &str, grid_constants: &'a GridConstants) -> Result<Grid<'a>, String> {
    let mut grid = Grid::init(grid_constants);
    let mut count_cells = 0;
    for c in grid_str.chars() {
        if c.is_digit(10) || c == '.' {
            if c != '0' && c != '.' {
                let value = c.to_digit(10).unwrap();
                if count_cells >= 81 {
                    return Err("Number of cells larger than 81".to_string());
                }
                let cell = grid_constants.cells[count_cells];
                if !grid.assign(&cell, value as u8) {
                    return Err("Unsolvable sudoku".to_string());
                }
            }
            count_cells += 1;
        }
    }
    if count_cells == 81 {
        Ok(grid)
    } else {
        Err(format!("Invalid number of cells: {}", count_cells))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let grid_constants = GridConstants::init();
        let grid_str = "4 . . |. . . |8 . 5 
                              . 3 . |. . . |. . . 
                              . . . |7 . . |. . . 
                              ------+------+------
                              . 2 . |. . . |. 6 . 
                              . . . |. 8 . |4 . . 
                              . . . |. 1 . |. . . 
                              ------+------+------
                              . . . |6 . 3 |. 7 . 
                              5 . . |2 . . |. . . 
                              1 . 4 |. . . |. . . ";
        let grid = parse_grid(grid_str, &grid_constants).unwrap();
        assert_eq!(vec![4], *grid.values(&('A', 1)));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], *grid.values(&('A', 3)));
        assert_eq!(vec![3], *grid.values(&('B', 2)));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], *grid.values(&('I', 9)));
    }
}