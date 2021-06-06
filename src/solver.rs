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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), String>{
        let samples = [
            ("4....927...5.1........6.9.8..1.86...6..4.......9..1.5....348.........132....7....", "416839275985217643237564918751986324628453791349721856192348567874695132563172489"),
            ("..5.4.2..9.1......37...86.5........17..8...3.6..2..7...........5...9...3.894....6", "865347219941625378372918645293754861754861932618239754436582197527196483189473526"),
            ("8.....29.62..............78....14....1.37.82..7...2.6......3..6.6....5..93.84....", "853467291627198435491235678289614357516379824374582169142753986768921543935846712"),
        ];

        for (input, output) in &samples {
            let in_grid = grid::from_line(input);
            let out_grid = grid::from_line(output);
            match solve(in_grid, 0, 0) {
                Ok(solution) => assert_eq!(solution, out_grid),
                Err(in_grid) => return Err(format!("Couldn't find solution for {}", in_grid)),
            };
        }
        Ok(())
    }
}