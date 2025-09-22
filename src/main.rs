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
       // outer layout: split vertically into two setions
       let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
            .split(size);

        // top section: welcome message
        let welcome = Paragraph::new("welcome to brew buddy!")
            .block(Block::default().borders(Borders::ALL).title("brew buddy"))
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(welcome, outer_layout[0]);

        // bottom section inner layout (split horizontally)
        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(outer_layout[1]);

        // left inner widget: counter placeholder test thing
        let counter_text = format!("counter: {}", self.counter);
        let counter = Paragraph::new(counter_text)
            .block(Block::default().borders(Borders::ALL).title("counter"))
            .style(Style::default().fg(Color::Cyan));
        frame.render_widget(counter, inner_layout[0]);

        // right inner widget: example placeholder
        let inner_right = Paragraph::new("inner 1")
            .block(Block::default().borders(Borders::ALL).title("placeholder"))
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(inner_right, inner_layout[1]); 
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