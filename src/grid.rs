macro_rules! sqr_idx {
    ($i:expr,$j:expr) => {{
        (($i / 3) * 3) + ($j / 3)
    }};
}

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<u8>>,

    rows_presence: Vec<u16>,
    cols_presence: Vec<u16>,
    sqrs_presence: Vec<u16>,
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

    pub fn to_line(&self) -> String {
        let bytes = self
            .grid
            .iter()
            .flatten()
            .map(|num| {
                if *num >= 1 && *num <= 9 {
                    return b'0' + num;
                }
                b'.'
            })
            .collect();
        String::from_utf8(bytes).unwrap()
    }
}

pub fn from_line(line: &str) -> Grid {
    let mut grid = Grid {
        grid: vec![vec![0; 9]; 9],

        rows_presence: vec![0; 9],
        cols_presence: vec![0; 9],
        sqrs_presence: vec![0; 9],
    };

    line.bytes().enumerate().for_each(|(idx, chr)| {
        if chr >= b'1' && chr <= b'9' {
            grid.set(idx / 9, idx % 9, chr - b'0');
        }
    });

    grid
}
