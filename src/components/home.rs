use color_eyre::{
    Result,
    owo_colors::{Color, OwoColorize},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph, Widget},
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, component::Component, config::Config};

use super::ConstrainedComponent;

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,

    components: Vec<Box<dyn ConstrainedComponent>>,
}

impl Home {
    pub fn new() -> Self {
        Self {
            command_tx: Option::default(),
            config: Config::default(),
            components: vec![Box::new(InputKindWidget::default())],
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx.clone());

        for c in self.components.iter_mut() {
            c.register_action_handler(tx.clone())?;
        }

        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config.clone();

        for c in self.components.iter_mut() {
            c.register_config_handler(config.clone())?;
        }

        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        // match action {
        //     Action::Tick => {
        //         tracing::info!("Home: Tick");
        //         // trace_dbg!("Home: Tick");
        //         // add any logic here that should run on every tick
        //     }
        //     Action::Render => {
        //         // add any logic here that should run on every render
        //     }
        //     _ => {}
        // }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3)].as_ref())
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                self.components[0].constraint(),
                Constraint::Length(8),
                Constraint::Length(9),
                Constraint::Length(16),
            ])
            .split(chunks[0]);

        let b = Paragraph::new("[ be ]").block(Block::default()).centered();
        let c = Paragraph::new("[ u32 ]").block(Block::default());
        let d = Paragraph::new("[ 0x01020304 ]").block(Block::default());

        self.components[0].draw(frame, horizontal_chunks[0])?;

        // frame.render_widget(a, horizontal_chunks[0]);
        frame.render_widget(b, horizontal_chunks[1]);
        frame.render_widget(c, horizontal_chunks[2]);
        frame.render_widget(d, horizontal_chunks[3]);

        // frame.render_widget(todo!(), chunks[0]);
        // Paragraph::new("hello world"), area);
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default)]
enum InputKind {
    Hex,
    #[default]
    Dec,
    Oct,
    Bin,
}

#[derive(Serialize, Deserialize)]
struct InputKindState {
    kind: InputKind,
    active: bool,
}

impl Default for InputKindState {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            active: true,
        }
    }
}

#[derive(Default)]
struct InputKindWidget {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,

    state: InputKindState,
}

impl ConstrainedComponent for InputKindWidget {
    fn constraint(&self) -> Constraint {
        Constraint::Length(9)
    }
}

impl Component for InputKindWidget {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        // match action {
        //     Action::Tick => {
        //         tracing::info!("Home: Tick");
        //         // trace_dbg!("Home: Tick");
        //         // add any logic here that should run on every tick
        //     }
        //     Action::Render => {
        //         // add any logic here that should run on every render
        //     }
        //     _ => {}
        // }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let style = if self.state.active {
            ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)
        } else {
            ratatui::style::Style::default()
        };

        let widget = Paragraph::new("[ HEX ]")
            .block(Block::default())
            .centered()
            .style(style);

        frame.render_widget(widget, area);
        // frame.render_widget(b, horizontal_chunks[1]);
        // frame.render_widget(c, horizontal_chunks[2]);
        // frame.render_widget(d, horizontal_chunks[3]);

        // frame.render_widget(todo!(), chunks[0]);
        // Paragraph::new("hello world"), area);
        Ok(())
    }
}
