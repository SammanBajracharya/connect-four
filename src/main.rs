mod board;
mod game;

use game::Game;

fn main() -> anyhow::Result<()> {
    let mut game = Game::new()?;
    game.run()?;

    Ok(())
}
