use colored::Colorize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    None,
    Red,
    Blue,
}

pub struct Board {
    board: [[Piece; 6]; 7],
    pub highlights: Option<usize>,
    pub cx: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Piece::None; 6]; 7],
            highlights: Some(0),
            cx: 0,
        }
    }

    pub fn get_board(&self) -> String {
        let mut string = String::new();

        string.push_str(" ╭ 1  2  3  4  5  6  7 ╮ \n");
        for y in 0..6 {
            string.push_str(" │");
            for x in 0..7 {
                let border = match self.highlights {
                    Some(col) => if col == x { "┊" } else { " " },
                    None => " ",
                };

                let icon = match self.highlights {
                    Some(col) => if col == x { "◈" } else { "◉" },
                    None => "◉",
                };

                let piece_str = match self.board[x][y] {
                    Piece::None => &format!("{1}{0}{1}", "◯".dimmed(), border),
                    Piece::Red => &format!("{1}{0}{1}", icon.red(), border),
                    Piece::Blue => &format!("{1}{0}{1}", icon.blue(), border),
                };

                string.push_str(piece_str);
            }
            string.push_str("│ \n");
        }
        string.push_str(" ╰");
        for x in 0..7 {
            match self.highlights {
                Some(col) => if col == x {
                    string.push_str("┴─┴")
                } else {
                    string.push_str("───")
                },
                None => string.push_str("───"),
            }
        }
        string.push_str("╯ ");
        string
    }

    pub fn drop_piece(&mut self, x: usize, piece: Piece) {
        for y in (0..6).rev() {
            match self.board[x][y] {
                Piece::None => {
                    self.board[x][y] = piece;
                    break;
                },
                _ => continue,
            }
        }
    }

    pub fn check_for_win(&mut self, x: usize) -> bool {
        let directions: [[(isize, isize); 2]; 4] = [
            [(-1, -1), (1, 1)],   // Diagonal (↘↖)
            [(1, -1), (-1, 1)],   // Anti-diagonal (↗↙)
            [(-1, 0), (1, 0)],    // Horizontal (←→)
            [(0, -1), (0, 1)],    // Vertical (↑↓)
        ];

        let mut y = 0;
        for i in 0..6 {
            if self.board[x][i] != Piece::None {
                y = i;
                break;
            }
        }

        for dir_pair in directions.iter() {
            let mut count = 1;

            for &(dx, dy) in dir_pair.iter() {
                let mut curr_x = x as isize;
                let mut curr_y = y as isize;

                for _ in 0..3 {
                    curr_x += dx;
                    curr_y += dy;

                    if !(0..7).contains(&curr_x) || !(0..6).contains(&curr_y) { break; }
                    if self.board[x][y] != self.board[curr_x as usize][curr_y as usize] { break; }


                    count += 1;
                }
            }
            if count >= 4 { return true; }
        }

        false
    }

    pub fn is_board_full(&self) -> bool {
        for x in 0..7 {
            if self.board[x][0] == Piece::None {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_win() {
        let mut board = Board::new();
        board.drop_piece(0, Piece::Red);
        board.drop_piece(0, Piece::Red);
        board.drop_piece(0, Piece::Red);
        board.drop_piece(0, Piece::Red);
        assert!(board.check_for_win(0));
    }

    #[test]
    fn test_horizontal_win() {
        let mut board = Board::new();
        board.drop_piece(0, Piece::Red);
        board.drop_piece(1, Piece::Red);
        board.drop_piece(2, Piece::Red);
        board.drop_piece(3, Piece::Red);
        assert!(board.check_for_win(3));
    }

    #[test]
    fn test_diagonal_win() {
        let mut board = Board::new();
        board.drop_piece(0, Piece::Red);
        board.drop_piece(0, Piece::Blue);
        board.drop_piece(0, Piece::Blue);
        board.drop_piece(0, Piece::Red);

        board.drop_piece(1, Piece::Blue);
        board.drop_piece(1, Piece::Blue);
        board.drop_piece(1, Piece::Red);

        board.drop_piece(2, Piece::Blue);
        board.drop_piece(2, Piece::Red);

        board.drop_piece(3, Piece::Red);

        assert!(board.check_for_win(3));
    }

    #[test]
    fn test_anti_diagonal_win() {
        let mut board = Board::new();
        board.drop_piece(3, Piece::Red);
        board.drop_piece(3, Piece::Blue);
        board.drop_piece(3, Piece::Blue);
        board.drop_piece(3, Piece::Blue);

        board.drop_piece(2, Piece::Blue);
        board.drop_piece(2, Piece::Blue);
        board.drop_piece(2, Piece::Red);

        board.drop_piece(1, Piece::Blue);
        board.drop_piece(1, Piece::Red);

        board.drop_piece(0, Piece::Red);

        assert!(board.check_for_win(0));
    }

    #[test]
    fn test_no_win() {
        let mut board = Board::new();
        board.drop_piece(0, Piece::Red);
        board.drop_piece(1, Piece::Blue);
        board.drop_piece(2, Piece::Red);
        assert!(!board.check_for_win(2));
    }

    #[test]
    fn test_board_full() {
        let mut board = Board::new();
        for x in 0..7 {
            for _ in 0..6 {
                board.drop_piece(x, Piece::Red);
            }
        }
        assert!(board.is_board_full());
    }

    #[test]
    fn test_board_not_full() {
        let mut board = Board::new();
        board.drop_piece(0, Piece::Red);
        board.drop_piece(1, Piece::Blue);
        assert!(!board.is_board_full());
    }

    #[test]
    fn test_drop_piece_full_column() {
        let mut board = Board::new();
        for _ in 0..6 {
            board.drop_piece(0, Piece::Red);
        }
        board.drop_piece(0, Piece::Blue);
        assert_eq!(board.board[0][0], Piece::Red);
    }
}
