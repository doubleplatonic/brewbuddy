use std::io::{self, stdout};
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

use ratatui::widgets::Paragraph;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

// app state
#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    selected: usize,
}

impl App {
    // main loop
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {

        let mut stdout = stdout();
        enable_raw_mode()?; // enable raw mode for key events
        execute!(stdout, EnterAlternateScreen)?; // take over the terminal

        loop {
            // draw UI
            terminal.draw(|f| self.draw(f))?;

            // handle key presses
            self.handle_events()?;

            // exit if requested
            if self.exit {
                break;
            }
        }

        disable_raw_mode()?; // restore terminal
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    // draw UI
    fn draw(&self, frame: &mut Frame<>) {
        let size = frame.size();

        // outer layout: split vertically into three sections
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(size);

        // title block
        let title = Paragraph::new("brew buddy")
                .alignment(Alignment::Center);
        frame.render_widget(title, outer[0]);

        // tea menu block
        let teas = vec![
            ListItem::new("green tea"),
            ListItem::new("black tea"),
            ListItem::new("herbal tea"),
            ListItem::new("jasmine tea"),
            ListItem::new("[+ add a custom tea]"),
        ];

        let mut state = ListState::default();
        state.select(Some(self.selected.min(teas.len() - 1))); // make sure in bounds

        let tea_list = List::new(teas)
            .block(Block::default().title("tea menu").borders(Borders::ALL))
            .highlight_style(Style::default()
            .fg(Color::Black)
            .bg(Color::LightRed));
        frame.render_stateful_widget(tea_list, outer[1], &mut state);

        // footer block
        let footer = Block::default()
            .title("↑/↓ to move • enter to brew • q to quit")
            .borders(Borders::ALL);
        frame.render_widget(footer, outer[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Up => self.select_previous(),
                    KeyCode::Down => self.select_next(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    // proper methods for selection
    fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn select_next(&mut self) {
        self.selected += 1;
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    app.run(&mut terminal)
}