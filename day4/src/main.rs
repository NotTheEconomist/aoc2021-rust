use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct BingoCell {
    value: u32,
    marked: bool,
}
impl BingoCell {
    #[allow(unused)]
    fn new(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct BingoBoard([BingoCell; 25]);
impl Display for BingoBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = self
            .rows()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| {
                        if cell.marked {
                            format!("*{:<3}", cell.value)
                        } else {
                            format!("{:<4}", cell.value)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_str(&formatted)
    }
}
impl BingoBoard {
    fn new(values: [u32; 25]) -> Self {
        let mut cells: [BingoCell; 25] = Default::default();
        for i in 0..25 {
            cells[i].value = values[i];
        }
        Self(cells)
    }
    fn rows(&self) -> Vec<Vec<&BingoCell>> {
        vec![
            self.0[0..5].iter().collect(),
            self.0[5..10].iter().collect(),
            self.0[10..15].iter().collect(),
            self.0[15..20].iter().collect(),
            self.0[20..25].iter().collect(),
        ]
    }

    fn cols(&self) -> Vec<Vec<&BingoCell>> {
        (0..5)
            .map(|i| {
                self.rows()
                    .into_iter()
                    .map(|mut row| row.remove(i))
                    .collect::<Vec<&BingoCell>>()
            })
            .collect()
    }

    fn is_winner(&self) -> bool {
        let (rows, cols) = (self.rows(), self.cols());
        let mut lines = rows.iter().chain(cols.iter());
        lines.any(|line| line.iter().all(|cell| cell.marked))
    }

    fn mark_number(&mut self, number: u32) {
        for mut cell in self.0.iter_mut() {
            if cell.value == number {
                cell.marked = true;
            }
        }
    }

    fn unmarked_numbers(&self) -> Vec<&BingoCell> {
        self.0.iter().filter(|cell| !cell.marked).collect()
    }
}

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, PartialEq, Eq, Debug)]
struct Input {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl Input {
    fn parse(input: &'static str) -> Result<Self, String> {
        let mut lines = input.lines();
        let numbers: Vec<u32> = lines
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let mut boards: Vec<BingoBoard> = Vec::new();
        loop {
            // What follows is N many boards with blank lines separating them
            if lines.next().is_none() {
                break;
            }
            let mut boardlines = [0; 25];
            let mut i = 0;
            (0..5).for_each(|_| {
                lines
                    .next()
                    .expect("Invalid input")
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .for_each(|n| {
                        boardlines[i] = n;
                        i += 1
                    });
            });
            boards.push(BingoBoard::new(boardlines));
        }

        Ok(Self { numbers, boards })
    }
}

fn solve_part1(input: Input) -> Option<u32> {
    let mut boards = input.boards.clone();
    let numbers = input.numbers;
    for number in numbers {
        for board in boards.iter_mut() {
            board.mark_number(number);
            if board.is_winner() {
                let score = board
                    .unmarked_numbers()
                    .iter()
                    .map(|cell| cell.value)
                    .reduce(std::ops::Add::add)
                    .expect("board cannot be empty")
                    * number;
                return Some(score);
            }
        }
    }
    None
}

fn solve_part2(input: Input) -> Option<u32> {
    let mut boards = input.boards.clone();
    let numbers = input.numbers;
    let mut winners: u32 = 0;
    let total_boards = boards.len() as u32;
    for number in numbers {
        for board in boards.iter_mut() {
            if board.is_winner() {
                continue;
            }
            // println!("Marking {} on board:\n{}", number, board);
            board.mark_number(number);
            if board.is_winner() {
                winners += 1;
                if winners == total_boards {
                    let score = board
                        .unmarked_numbers()
                        .iter()
                        .map(|cell| cell.value)
                        .reduce(std::ops::Add::add)
                        .expect("board cannot be empty")
                        * number;
                    return Some(score);
                }
            }
        }
    }
    None
}

fn main() {
    let input = Input::parse(INPUT).expect("failed to parse input");
    let part1 = solve_part1(input.clone()).expect("invalid input");
    println!("part1: {}", part1);
    let part2 = solve_part2(input).expect("invalid input");
    println!("part2: {}", part2);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn test_solve_part1() {
        let input = Input::parse(INPUT).expect("failed to parse input");
        let score = solve_part1(input).expect("test game should finish with a winner");
        assert_eq!(score, 4512);
    }

    #[test]
    fn test_solve_part2() {
        let input = Input::parse(INPUT).expect("failed to parse input");
        let score = solve_part2(input).expect("test game should finish with a final winner");
        assert_eq!(score, 1924);
    }

    #[test]
    fn test_parse() {
        let got = Input::parse(INPUT).expect("Failed to parse input completely");
        let want = Input {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                BingoBoard::new([
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19,
                ]),
                BingoBoard::new([
                    3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21,
                    16, 12, 6,
                ]),
                BingoBoard::new([
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7,
                ]),
            ],
        };
        assert_eq!(got, want);
    }

    #[test]
    fn test_mark_board() {
        let mut board = BingoBoard::new([
            1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        ]);
        board.mark_number(1);
        assert!(board.0[0].marked);
    }

    #[test]
    fn test_win_condition() {
        let board = BingoBoard([
            BingoCell {
                value: 1,
                marked: false,
            },
            BingoCell {
                value: 6,
                marked: false,
            },
            BingoCell {
                value: 11,
                marked: false,
            },
            BingoCell {
                value: 16,
                marked: false,
            },
            BingoCell {
                value: 21,
                marked: false,
            },
            BingoCell {
                value: 2,
                marked: false,
            },
            BingoCell {
                value: 7,
                marked: false,
            },
            BingoCell {
                value: 12,
                marked: false,
            },
            BingoCell {
                value: 17,
                marked: false,
            },
            BingoCell {
                value: 22,
                marked: false,
            },
            BingoCell {
                value: 3,
                marked: false,
            },
            BingoCell {
                value: 8,
                marked: false,
            },
            BingoCell {
                value: 13,
                marked: false,
            },
            BingoCell {
                value: 18,
                marked: false,
            },
            BingoCell {
                value: 23,
                marked: false,
            },
            BingoCell {
                value: 4,
                marked: false,
            },
            BingoCell {
                value: 9,
                marked: false,
            },
            BingoCell {
                value: 14,
                marked: false,
            },
            BingoCell {
                value: 19,
                marked: false,
            },
            BingoCell {
                value: 24,
                marked: false,
            },
            BingoCell {
                value: 5,
                marked: false,
            },
            BingoCell {
                value: 10,
                marked: false,
            },
            BingoCell {
                value: 15,
                marked: false,
            },
            BingoCell {
                value: 20,
                marked: false,
            },
            BingoCell {
                value: 25,
                marked: false,
            },
        ]);

        assert!(!board.is_winner(), "new board should not win");

        let mut rowboard = board.clone();

        (0..5).for_each(|i| {
            rowboard.0[i].marked = true;
        });
        assert!(rowboard.is_winner(), "row board should win");

        let mut colboard = board;
        (0..5).for_each(|i| {
            colboard.0[i * 5].marked = true;
        });
        assert!(colboard.is_winner(), "col board should win");
    }

    #[test]
    fn test_cols() {
        let board = BingoBoard::new([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        let want_board = vec![
            vec![
                BingoCell::new(1),
                BingoCell::new(6),
                BingoCell::new(11),
                BingoCell::new(16),
                BingoCell::new(21),
            ],
            vec![
                BingoCell::new(2),
                BingoCell::new(7),
                BingoCell::new(12),
                BingoCell::new(17),
                BingoCell::new(22),
            ],
            vec![
                BingoCell::new(3),
                BingoCell::new(8),
                BingoCell::new(13),
                BingoCell::new(18),
                BingoCell::new(23),
            ],
            vec![
                BingoCell::new(4),
                BingoCell::new(9),
                BingoCell::new(14),
                BingoCell::new(19),
                BingoCell::new(24),
            ],
            vec![
                BingoCell::new(5),
                BingoCell::new(10),
                BingoCell::new(15),
                BingoCell::new(20),
                BingoCell::new(25),
            ],
        ];
        for (gotcol, wantcol) in board.cols().into_iter().zip(want_board.into_iter()) {
            for (gotcell, wantcell) in gotcol.into_iter().zip(wantcol.iter()) {
                assert_eq!(gotcell, wantcell);
            }
        }
    }
    #[test]
    fn test_rows() {
        let board = BingoBoard::new([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25,
        ]);
        let want_board = vec![
            vec![
                BingoCell::new(1),
                BingoCell::new(2),
                BingoCell::new(3),
                BingoCell::new(4),
                BingoCell::new(5),
            ],
            vec![
                BingoCell::new(6),
                BingoCell::new(7),
                BingoCell::new(8),
                BingoCell::new(9),
                BingoCell::new(10),
            ],
            vec![
                BingoCell::new(11),
                BingoCell::new(12),
                BingoCell::new(13),
                BingoCell::new(14),
                BingoCell::new(15),
            ],
            vec![
                BingoCell::new(16),
                BingoCell::new(17),
                BingoCell::new(18),
                BingoCell::new(19),
                BingoCell::new(20),
            ],
            vec![
                BingoCell::new(21),
                BingoCell::new(22),
                BingoCell::new(23),
                BingoCell::new(24),
                BingoCell::new(25),
            ],
        ];
        for (gotrow, wantrow) in board.rows().into_iter().zip(want_board.into_iter()) {
            for (gotcell, wantcell) in gotrow.into_iter().zip(wantrow.iter()) {
                assert_eq!(gotcell, wantcell);
            }
        }
    }
}
