mod board;
mod game;

use std::{io::stdout, panic};
use crossterm::{terminal, ExecutableCommand};

use game::Game;

fn main() -> anyhow::Result<()> {
    let mut game = Game::new()?;

    panic::set_hook(Box::new(|info| {
        _ = stdout().execute(terminal::LeaveAlternateScreen);
        _ = terminal::disable_raw_mode();
        eprintln!("{}", info);
    }));

    game.run()?;
    game.cleanup()?;

    Ok(())
}
