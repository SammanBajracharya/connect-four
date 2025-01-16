mod board;
mod game;

use game::Game;
use strip_ansi::strip_ansi;
use colored::*;
use unicode_segmentation::UnicodeSegmentation;

fn main() -> anyhow::Result<()> {
    let mut game = Game::new()?;
    game.run()?;
    game.cleanup()?;

    let string = "◉".red();
    let stripped_string_len = strip_ansi(string.trim()).len();
    let unicode_segmentation_len = string.graphemes(true).count();

    println!("{}", stripped_string_len);
    println!("{}", unicode_segmentation_len);
    Ok(())
}
