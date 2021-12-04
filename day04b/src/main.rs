struct Cell {
    value: u8,
    marked: bool,
}

impl Cell {
    fn new(value: u8) -> Self {
        Cell {
            value,
            marked: false,
        }
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
    size: usize,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|s| Cell::new(s.parse().unwrap()))
                    .collect()
            })
            .collect();

        let size = cells.len();

        Board { cells, size }
    }

    fn sum_unmarked(&self) -> u64 {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|cell| !cell.marked)
                    .map(|cell| cell.value as u64)
                    .sum::<u64>()
            })
            .sum()
    }

    fn mark(&mut self, value: u8) {
        for row in &mut self.cells {
            for cell in row {
                if cell.value == value {
                    cell.marked = true;
                }
            }
        }
    }

    fn solved(&self) -> bool {
        // check rows
        for row in &self.cells {
            if row.iter().all(|c| c.marked) {
                return true;
            }
        }

        // check columns`
        for i in 0..self.size {
            if self.cells.iter().map(|row| &row[i]).all(|c| c.marked) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let mut input = include_str!("../input.txt").split("\n\n");
    let draws: Vec<u8> = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let mut boards: Vec<_> = input.map(|x| Board::from_str(x)).collect();

    for draw in draws {
        for board in &mut boards {
            board.mark(draw);
        }
        if boards.iter().all(|board| board.solved()) {
            println!("{}", boards.last().unwrap().sum_unmarked() * draw as u64);
            break;
        }
        boards = boards.into_iter().filter(|board| !board.solved()).collect();
    }
}
