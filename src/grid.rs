macro_rules! sqr_idx {
    ($i:expr,$j:expr) => {{
        (($i / 3) * 3) + ($j / 3)
    }};
}

use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid {
    grid: [[u8; 9]; 9],

    rows_presence: [u16; 9],
    cols_presence: [u16; 9],
    sqrs_presence: [u16; 9],
}

impl Grid {
    pub fn set(&mut self, i: usize, j: usize, num: u8) {
        self.grid[i][j] = num;

        let bit_mask = 1u16 << (num - 1);
        self.rows_presence[i] |= bit_mask;
        self.cols_presence[j] |= bit_mask;
        self.sqrs_presence[sqr_idx!(i, j)] |= bit_mask;
    }

    pub fn unset(&mut self, i: usize, j: usize, num: u8) {
        self.grid[i][j] = 0;

        let bit_mask = !(1u16 << (num - 1));
        self.rows_presence[i] &= bit_mask;
        self.cols_presence[j] &= bit_mask;
        self.sqrs_presence[sqr_idx!(i, j)] &= bit_mask;
    }

    pub fn is_set(&self, i: usize, j: usize) -> bool {
        return self.grid[i][j] != 0;
    }

    pub fn available(&self, i: usize, j: usize) -> Vec<u8> {
        let bit_mask =
            self.rows_presence[i] | self.cols_presence[j] | self.sqrs_presence[sqr_idx!(i, j)];
        (0..9)
            .filter(move |num| ((bit_mask >> num) & 1) == 0)
            .map(|num| num + 1)
            .collect()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> std::fmt::Result {
        for num in self.grid.iter().flatten() {
            if *num != 0 {
                fmt.write_char((b'0' + num) as char)?;
            } else {
                fmt.write_char('.')?;
            }
        }
        Ok(())
    }
}

pub fn from_line(line: &str) -> Grid {
    let mut grid = Grid {
        grid: [[0u8; 9]; 9],

        rows_presence: [0u16; 9],
        cols_presence: [0u16; 9],
        sqrs_presence: [0u16; 9],
    };

    line.bytes().enumerate().for_each(|(idx, chr)| {
        if chr >= b'1' && chr <= b'9' {
            grid.set(idx / 9, idx % 9, chr - b'0');
        }
    });

    grid
}
