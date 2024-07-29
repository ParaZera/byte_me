//! # [Ratatui] Paragraph example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use std::{
    io::{self},
    time::{Duration, Instant},
};

use crossterm::event::KeyEventKind;
use ratatui::{
    buffer::Buffer,
    crossterm::{
        self,
        event::{self, Event, KeyCode},
    },
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use self::common::{init_terminal, install_hooks, restore_terminal, Tui};

fn main() -> color_eyre::Result<()> {
    install_hooks()?;
    let mut terminal = init_terminal()?;
    let mut app = App::new();
    app.run(&mut terminal)?;
    restore_terminal()?;
    Ok(())
}

#[derive(Debug)]
struct App {
    blub: Blub,
    should_exit: bool,
    scroll: u16,
    last_tick: Instant,
    number: u128,
}

impl App {
    /// The duration between each tick.
    const TICK_RATE: Duration = Duration::from_millis(100);

    /// Create a new instance of the app.
    fn new() -> Self {
        Self {
            should_exit: false,
            scroll: 0,
            last_tick: Instant::now(),
            blub: Blub {},
            number: 0,
        }
    }

    /// Run the app until the user exits.
    fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.should_exit {
            self.draw(terminal)?;
            self.handle_events()?;
            if self.last_tick.elapsed() >= Self::TICK_RATE {
                self.on_tick();
                self.last_tick = Instant::now();
            }
        }
        Ok(())
    }

    /// Draw the app to the terminal.
    fn draw(&mut self, terminal: &mut Tui) -> io::Result<()> {
        terminal.draw(|frame| frame.render_widget(self, frame.size()))?;
        Ok(())
    }

    fn is_hex_character(char: KeyCode) -> bool {
        if let KeyCode::Char(char) = char {
            return match char {
                '0'..='9' | 'a'..='f' | 'A'..='F' => true,
                _ => false,
            };
        }

        false
    }

    /// Handle events from the terminal.
    fn handle_events(&mut self) -> io::Result<()> {
        let timeout = Self::TICK_RATE.saturating_sub(self.last_tick.elapsed());
        while event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.should_exit = true;
                }

                if key.kind == KeyEventKind::Press && App::is_hex_character(key.code) {
                    if let KeyCode::Char(char) = key.code {
                        self.number = u128::from_str_radix(&char.to_string(), 16).unwrap();
                    }
                }
            }
        }
        Ok(())
    }

    /// Update the app state on each tick.
    fn on_tick(&mut self) {
        self.scroll = (self.scroll + 1) % 10;
    }
}

#[derive(Debug)]
struct Blub;

impl Widget for &mut Blub {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(vec![
            Line::raw(" 128        96        64        32        ")
                .alignment(Alignment::Right)
                .dim(),
            Line::raw("  1234 1234 1234 1234 1234 1234 1234 1235 ")
                .alignment(Alignment::Right)
                .bold(),
        ])
        .block(
            Block::bordered()
                .title(" i128 ")
                .title_alignment(Alignment::Left)
                .title_style(Style::new().bold())
                .border_type(BorderType::Double),
        )
        .render(area, buf)
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
            .flex(Flex::Center)
            .split(area);

        let sub_areas = Layout::horizontal([Constraint::Length(3), Constraint::Max(128 + 32 + 2)])
            .flex(Flex::Center)
            .split(areas[0]);

        Paragraph::new(vec![Line::raw("0x").alignment(Alignment::Center)])
            .block(
                Block::new()
                    .border_set(symbols::border::PLAIN)
                    .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM),
            )
            .render(sub_areas[0], buf);
        Paragraph::new(vec![Line::raw("Input Stuff").alignment(Alignment::Center)])
            .block(
                Block::new()
                    .border_set(symbols::border::PLAIN)
                    .borders(Borders::LEFT | Borders::BOTTOM | Borders::TOP | Borders::RIGHT),
            )
            .render(sub_areas[1], buf);

        self.blub.render(areas[1], buf);
    }
}

/// A module for common functionality used in the examples.
mod common {
    use std::{
        io::{self, stdout, Stdout},
        panic,
    };

    use color_eyre::{
        config::{EyreHook, HookBuilder, PanicHook},
        eyre,
    };
    use crossterm::ExecutableCommand;
    use ratatui::{
        backend::CrosstermBackend,
        crossterm::terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        terminal::Terminal,
    };

    // A simple alias for the terminal type used in this example.
    pub type Tui = Terminal<CrosstermBackend<Stdout>>;

    /// Initialize the terminal and enter alternate screen mode.
    pub fn init_terminal() -> io::Result<Tui> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        Terminal::new(backend)
    }

    /// Restore the terminal to its original state.
    pub fn restore_terminal() -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Installs hooks for panic and error handling.
    ///
    /// Makes the app resilient to panics and errors by restoring the terminal before printing the
    /// panic or error message. This prevents error messages from being messed up by the terminal
    /// state.
    pub fn install_hooks() -> color_eyre::Result<()> {
        let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();
        install_panic_hook(panic_hook);
        install_error_hook(eyre_hook)?;
        Ok(())
    }

    /// Install a panic hook that restores the terminal before printing the panic.
    fn install_panic_hook(panic_hook: PanicHook) {
        let panic_hook = panic_hook.into_panic_hook();
        panic::set_hook(Box::new(move |panic_info| {
            let _ = restore_terminal();
            panic_hook(panic_info);
        }));
    }

    /// Install an error hook that restores the terminal before printing the error.
    fn install_error_hook(eyre_hook: EyreHook) -> color_eyre::Result<()> {
        let eyre_hook = eyre_hook.into_eyre_hook();
        eyre::set_hook(Box::new(move |error| {
            let _ = restore_terminal();
            eyre_hook(error)
        }))?;
        Ok(())
    }
}
