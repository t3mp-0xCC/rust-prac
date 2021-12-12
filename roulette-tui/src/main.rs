use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, stdout},
    path::Path
};
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table,},
    Terminal
};
use rand::Rng;

fn main() -> Result<(), io::Error>{
    // show help
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: roulette <elements file>\nexp: roulette foods.txt");
        return Ok(());
    }

    // reading file and get elements as vec
    let path = Path::new(&args[1]);
    let roulette_elements = lines_from_file(path)
        .expect("file not found !");

    //init
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;    

    // draw TUI
    loop {
        terminal.draw(|f| {
            let chunks0 = Layout::default()
                .margin(1)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Percentage(80),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let text = Text::from("Simple Roulette");
            f.render_widget(
                Paragraph::new(text).style(Style::default().add_modifier(Modifier::BOLD)),
                chunks0[0],
            );
        })?;
    }

    Ok(())

}

// reading file and split line into vec
fn lines_from_file(file: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}
