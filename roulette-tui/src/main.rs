use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, stdout},
    path::Path, cell
};
use termion::raw::IntoRawMode;
use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{BarChart, Block, Borders,Cell, List, ListItem, Paragraph, Row, Table, TableState, Wrap,},
    Terminal
};
use rand::Rng;
use unicode_width::UnicodeWidthStr;

/*TODO: show list of elements*/
fn main() -> Result<(), io::Error>{
    // show help
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: roulette <elements file>\nexp: roulette foods.txt");
        return Ok(());
    }

    // reading file and get elements as vec
    let path = Path::new(&args[1]);
    let file =  lines_from_file(path);

    //init
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;    

    // draw TUI
    terminal.draw(|f| {
        let chunks0 = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Percentage(90),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(f.size());
        
        // title
        let text = Text::from("Simple Roulette");            
        f.render_widget(
            Paragraph::new(text).style(Style::default().add_modifier(Modifier::BOLD)),
            chunks0[0],
        );

        // make chunks for list
        let chunks1 =  Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(100),
                ].as_ref()
            ).split(chunks0[1]);

        // elements list block
        let block = Block::default().title("elements").borders(Borders::ALL);
        f.render_widget(block, chunks1[0]);
        // make vector of elements
        let elements: Vec<ListItem> = file
            .iter()
            .enumerate()
            .map(|(i, m)|{
                let element = Spans::from(Span::raw(format!("{}: {:?}", i, m)));
                ListItem::new(element)
            })
            .collect();

        // make list
        let element_list = List::new(elements)
        .block(Block::default().borders(Borders::ALL)
            .title("elements")
        )
        .highlight_symbol(">> ");
        f.render_widget(element_list, chunks1[0]);
        

        // result
        let text = Text::from("result");
        f.render_widget(
            Paragraph::new(text).style(Style::default().bg(Color::White).fg(Color::Black)),
            chunks0[2],
        );

    })?;

    Ok(())

}

// reading file and split line into vec
fn lines_from_file(file: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}

