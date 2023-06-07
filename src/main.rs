use colored::Colorize;
use std::fmt;

struct Sudoku {
    grid: [[u8; 9]; 9],
    givens: Vec<(usize, usize)>,
}
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for (i, row) in self.grid.iter().enumerate() {
            if (i % 3 == 0) & (i != 0) {
                writeln!(f, "├───────┼───────┼───────┤")?;
            }
            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 {
                    write!(f, "│ ")?;
                }
                if self.givens.contains(&(i, j)) {
                    write!(f, "{} ", cell.to_string().yellow())?;
                } else {
                    write!(f, "{} ", cell)?;
                }
            }
            writeln!(f, "│")?;
        }
        writeln!(f, "└───────┴───────┴───────┘")
    }
}
fn main() {
    let grid: [[u8; 9]; 9] = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [2, 3, 4, 5, 6, 7, 8, 9, 1],
        [5, 6, 7, 8, 9, 1, 2, 3, 4],
        [8, 9, 1, 2, 3, 4, 5, 6, 7],
        [3, 4, 5, 6, 7, 8, 9, 1, 2],
        [6, 7, 8, 9, 1, 2, 3, 4, 5],
        [9, 1, 2, 3, 4, 5, 6, 7, 8],
    ];
    let givens: Vec<(usize, usize)> = vec![(0, 0), (3, 3), (4, 6)];
    let s: Sudoku = Sudoku { grid, givens };
    println!("{:}", s);
}
