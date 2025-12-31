use std::io;

mod terminal;
mod main_menu;

fn main() -> io::Result<()> {

    let mut terminal = terminal::setup_terminal()?;

    let res = main_menu::run_app(&mut terminal);

    terminal::restore_terminal(&mut terminal)?;
    res
}
