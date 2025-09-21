use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    Frame,
};

// app state
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    // main loop
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        enable_raw_mode()?; // enable raw mode for key events

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
        Ok(())
    }

    // draw UI
    fn draw(&self, frame: &mut Frame<>) {
        let size = frame.size();

        // layout: split vertically into two sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(size);

        // welcome message!
        let welcome = Paragraph::new("welcome to brew buddy! ^_^")
            .block(Block::default().borders(Borders::ALL).title("brew buddie"))
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(welcome, chunks[0]);

        // counter display (placeholder)
        let counter_text = format!("counter: {}", self.counter);
        let counter = Paragraph::new(counter_text)
            .block(Block::default().borders(Borders::ALL).title("counter"))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(counter, chunks[1]);
    }

    // handle key events
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,                     // quit
                    KeyCode::Right => self.counter = self.counter.saturating_add(1), // increment
                    KeyCode::Left => self.counter = self.counter.saturating_sub(1), // decrement
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    app.run(&mut terminal)
}