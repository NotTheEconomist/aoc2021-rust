#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
struct BingoCell {
    value: u32,
    marked: bool,
}
impl BingoCell {
    fn new(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Clone)]
struct BingoBoard([BingoCell; 25]);
impl BingoBoard {
    fn new(values: [u32; 25]) -> Self {
        let mut cells: [BingoCell; 25] = Default::default();
        for i in 0..25 {
            cells[i].value = values[i];
        }
        Self(cells)
    }
    fn rows(&self) -> Vec<Vec<BingoCell>> {
        vec![
            self.0[0..5].to_vec(),
            self.0[5..10].to_vec(),
            self.0[10..15].to_vec(),
            self.0[15..20].to_vec(),
            self.0[20..25].to_vec(),
        ]
    }

    fn cols(&self) -> Vec<Vec<BingoCell>> {
        (0..5)
            .map(|i| {
                self.rows()
                    .iter()
                    .map(|row| *row.get(i).unwrap())
                    .collect::<Vec<BingoCell>>()
            })
            .collect()
    }

    fn is_winner(&self) -> bool {
        let (rows, cols) = (self.rows(), self.cols());
        let mut lines = rows.iter().chain(cols.iter());
        lines.any(|line| line.iter().all(|cell| cell.marked))
    }

    fn mark_number(&self, number: u32) {
        for mut cell in self.0 {
            if cell.value == number {
                cell.marked = true;
            }
        }
    }

    fn unmarked_numbers(&self) -> Vec<BingoCell> {
        self.0.into_iter().filter(|cell| !cell.marked).collect()
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

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
        for (gotcol, wantcol) in board.cols().iter().zip(want_board.iter()) {
            for (gotcell, wantcell) in gotcol.iter().zip(wantcol.iter()) {
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
        for (gotrow, wantrow) in board.rows().iter().zip(want_board.iter()) {
            for (gotcell, wantcell) in gotrow.iter().zip(wantrow.iter()) {
                assert_eq!(gotcell, wantcell);
            }
        }
    }
}
