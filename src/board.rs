use colored::Colorize;

#[derive(Copy, Clone, PartialEq)]
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

pub fn get_padding(text: String, width: usize) -> (usize, usize) {
    let left_padding = (width - text.len()) / 2;
    let right_padding = width - text.len() - left_padding;

    (left_padding, right_padding)
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
            string.push_str(" |");
            for x in 0..7 {
                let border = match self.highlights {
                    Some(col) => if col == x { "╏" } else { " " },
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
            string.push_str("| \n");
        }
        string.push_str(" ╰─────────────────────╯ ");
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

    //pub fn highlight_col(&mut self, x: usize) {
    //    self.highlights = [[fa;
    //    for y in 0..6 {
    //        self.highlights[x][y] = true;
    //    }
    //}

    pub fn check_for_win(&mut self, x: usize, piece: Piece) -> bool {
        let check_bound: [[(isize, isize); 2]; 4] = [
            [(-1, -1), (1, 1)], // Diagonal top-left to bottom-right
            [(1, -1), (-1, 1)], // Diagonal bottom-left to top-right
            [(1, 0), (-1, 0)],  // Horizontal
            [(0, 1), (0, -1)],  // Vertical
        ];

        let mut y = None;
        for row in 0..6 {
            if self.board[x][row] == piece { y = Some(row); }
        }

        let y = match y {
            Some(y) => y,
            None => return false,
        };

        for &directions in &check_bound {
            let mut count = 1;

            for &(dx, dy) in directions.iter() {
                let mut curr_x = x as isize;
                let mut curr_y = y as isize;

                while (0..7).contains(&curr_x) && (0..6).contains(&curr_y) {
                    if self.board[curr_x as usize][curr_y as usize] != piece {
                        break;
                    }
                    count += 1;
                    curr_x += dx;
                    curr_y += dy;
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
