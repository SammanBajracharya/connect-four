mod board;

use board::{Board, Piece};

fn main() {
    let mut board = Board::new();
    board.print();

    board.drop_piece(3, Piece::Blue);
    board.drop_piece(3, Piece::Red);
    board.drop_piece(3, Piece::Blue);
    board.highlight_col(3);
    board.print();
}
