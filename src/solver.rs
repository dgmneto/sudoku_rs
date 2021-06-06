use crate::grid;

pub fn solve(mut grid: grid::Grid, i: usize, j: usize) -> Result<grid::Grid, grid::Grid> {
    if i >= 9 {
        return Ok(grid);
    }

    let mut next_i = i;
    let mut next_j = j + 1;
    if next_j == 9 {
        next_i += 1;
        next_j = 0;
    }
    if grid.is_set(i, j) {
        return solve(grid, next_i, next_j);
    }
    let availables = grid.available(i, j);
    for num in availables {
        grid.set(i, j, num);
        grid = match solve(grid, next_i, next_j) {
            Ok(grid) => return Ok(grid),
            Err(grid) => grid,
        };
        grid.unset(i, j, num);
    }

    Err(grid)
}
