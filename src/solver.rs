use crate::grid;

fn solve_impl<'b>(
    order: &[(usize, usize); 81],
    grid: &'b mut grid::Grid,
    n: usize,
) -> Result<&'b mut grid::Grid, &'b mut grid::Grid> {
    if n >= 81 {
        return Ok(grid);
    }

    let mut grid = grid;
    let (i, j) = order[n];
    if grid.is_set(i, j) {
        return solve_impl(order, grid, n + 1);
    }
    let availables = grid.available(i, j);
    for num in availables {
        grid.set(i, j, num);
        grid = match solve_impl(order, grid, n + 1) {
            Ok(grid) => return Ok(grid),
            Err(grid) => grid,
        };
        grid.unset(i, j, num);
    }

    Err(grid)
}

#[inline]
fn order(grid: &mut grid::Grid, sort: bool) -> [(usize, usize); 81] {
    let mut rows = [0; 9];
    for row in 0..9 {
        rows[row] = row;
    }
    let mut cols = rows.clone();
    if sort {
        rows.sort_unstable_by_key(|row| grid.count_unset_row(row));
        cols.sort_unstable_by_key(|col| grid.count_unset_col(col));
    }
    let mut order = [(0, 0); 81];
    let mut idx = 0;
    for row in &rows {
        for col in &cols {
            order[idx] = (*row, *col);
            idx += 1;
        }
    }
    order
}

pub fn solve<'a>(
    grid: &'a mut grid::Grid,
    sort_solving: bool,
) -> Result<&'a mut grid::Grid, &'a mut grid::Grid> {
    let order = order(grid, sort_solving);
    match solve_impl(&order, grid, 0) {
        Ok(grid) => Ok(grid),
        Err(grid) => Err(grid),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: [(&str, &str); 3] = [
        (
            "4....927...5.1........6.9.8..1.86...6..4.......9..1.5....348.........132....7....",
            "416839275985217643237564918751986324628453791349721856192348567874695132563172489",
        ),
        (
            "..5.4.2..9.1......37...86.5........17..8...3.6..2..7...........5...9...3.894....6",
            "865347219941625378372918645293754861754861932618239754436582197527196483189473526",
        ),
        (
            "8.....29.62..............78....14....1.37.82..7...2.6......3..6.6....5..93.84....",
            "853467291627198435491235678289614357516379824374582169142753986768921543935846712",
        ),
    ];

    #[test]
    fn test_naive() -> Result<(), String> {
        for (input, output) in &SAMPLES {
            let mut in_grid = grid::from_line(input);
            let mut out_grid = grid::from_line(output);
            match solve(&mut in_grid, false) {
                Ok(solution) => assert_eq!(solution, &mut out_grid),
                Err(in_grid) => return Err(format!("Couldn't find solution for {}", in_grid)),
            };
        }

        Ok(())
    }

    #[test]
    fn test_sorted() -> Result<(), String> {
        for (input, output) in &SAMPLES {
            let mut in_grid = grid::from_line(input);
            let mut out_grid = grid::from_line(output);
            match solve(&mut in_grid, true) {
                Ok(solution) => assert_eq!(solution, &mut out_grid),
                Err(in_grid) => return Err(format!("Couldn't find solution for {}", in_grid)),
            };
        }

        Ok(())
    }
}
