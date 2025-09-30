use std::io::{self, stdout};
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

// app state
#[derive(Debug)]
pub struct App {
    exit: bool,
    selected: usize,
    progressbar: u16, // 0-100
    steeping: bool,
    last_tick: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            selected: 0,
            progressbar: 0,
            steeping: false,
            last_tick: Instant::now(),
        }
    }
}

impl App {
    // main loop
    pub fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        let mut stdout = stdout();
        enable_raw_mode()?; // enable raw mode for key events
        execute!(stdout, EnterAlternateScreen)?; // take over the terminal

        while !self.exit {
            // draw UI
            terminal.draw(|f| self.draw(f))?;

            // handle key presses
            self.handle_events()?;

            // update progress on tick
            self.update_progress();
        }

        disable_raw_mode()?; // restore terminal
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        Ok(())
    }

    // draw UI
    fn draw(&self, frame: &mut Frame) {
        let size = frame.size();

        // outer layout
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3), // title
                Constraint::Min(10),   // main content
                Constraint::Length(3), // footer
            ])
            .split(size);

        // title block
        let title =
            Paragraph::new("welcome to brew buddy: your tea brewing companion!").alignment(Alignment::Center);
        frame.render_widget(title, outer[0]);

        // middle section
        let middle = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer[1]);

        // tea menu
        let teas = vec![
            ListItem::new("green tea"),
            ListItem::new("black tea"),
            ListItem::new("herbal tea"),
            ListItem::new("jasmine tea"),
            ListItem::new("[+ add a custom tea]"),
        ];

        let mut state = ListState::default();
        state.select(Some(self.selected.min(teas.len() - 1)));

        let tea_list = List::new(teas)
            .block(Block::default().title("tea menu").borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::Black).bg(Color::LightRed));
        frame.render_stateful_widget(tea_list, middle[0], &mut state);

        // tea progress gauge
        let gauge = Gauge::default()
            .block(Block::default().title("tea progress").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(self.progressbar as f64 / 100.0);
        frame.render_widget(gauge, outer[1]);

        // tea journal
        let tea_journal = Block::default().title("tea journal").borders(Borders::ALL);
        frame.render_widget(tea_journal, middle[1]);

        // footer
        let footer = Paragraph::new("↑/↓ to move • enter to brew • q to quit").alignment(Alignment::Center);
        frame.render_widget(footer, outer[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Up => self.select_previous(),
                    KeyCode::Down => self.select_next(),
                    KeyCode::Enter => {
                        if !self.steeping {
                            self.steeping = true;
                            self.progressbar = 0;
                        }
                    }
                    KeyCode::Char('b') => self.exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn update_progress(&mut self) {
        if self.steeping && self.last_tick.elapsed() >= Duration::from_millis(100) {
            self.last_tick = Instant::now();
            if self.progressbar < 100 {
                self.progressbar += 1;
            } else {
                self.steeping = false;
            }
        }
    }

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