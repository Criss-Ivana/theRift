use std::io;

mod terminal;
mod app;
mod screens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    Playing,
    Quit,
}

fn main() -> io::Result<()> {

    let mut terminal = terminal::setup_terminal()?;

    let res = app::run_app(&mut terminal);

    terminal::restore_terminal(&mut terminal)?;
    res
}
