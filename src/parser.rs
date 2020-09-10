use crate::solver::Grid;

pub fn parse_grid(grid_str: &str) -> Grid {
    let mut grid = Grid::init();
    let mut count_cells = 0;
    for c in grid_str.chars() {
        if c.is_digit(10) || c == '.' {
            if c != '0' && c != '.' {
                let value = c.to_digit(10).unwrap();
                if count_cells >= 81 {
                    panic!("Number of cells passes 81");
                }
                let cell = crate::CELLS.get(count_cells).unwrap();
                if !grid.assign(*cell, value as u8) {
                    panic!("Unsolvable sudoku");
                }
            }
            count_cells += 1;
        }
    }
    if count_cells != 81 {
        panic!("Invalid number of cells");
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
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
        let grid = parse_grid(grid_str);
        assert_eq!(vec![4], *grid.values(&('A', 1)));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], *grid.values(&('A', 3)));
        assert_eq!(vec![3], *grid.values(&('B', 2)));
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], *grid.values(&('I', 9)));
    }
}