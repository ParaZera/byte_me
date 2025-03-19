use color_eyre::Result;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, List, Paragraph, Widget},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, component::Component, config::Config};

trait ConstrainedWidget: Widget {
    fn constraint(&self) -> Constraint;
}

#[derive(Default)]
pub struct HeaderState {}

#[derive(Default)]
pub struct Header {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,

    state: HeaderState,
    components: Vec<Box<dyn Component>>,
}

impl Header {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Header {
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
        let chunks = Layout::default()
            .constraints([Constraint::Length(3)].as_ref())
            .split(area);

        let horizontal_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Length(9),
                Constraint::Length(8),
                Constraint::Length(9),
                Constraint::Length(16),
            ])
            .split(chunks[0]);

        let a = Paragraph::new("[ hex ]").block(Block::default()).centered();
        let b = Paragraph::new("[ be ]").block(Block::default()).centered();
        let c = Paragraph::new("[ u32 ]").block(Block::default());
        let d = Paragraph::new("[ 0x01020304 ]").block(Block::default());

        frame.render_widget(a, horizontal_chunks[0]);
        frame.render_widget(b, horizontal_chunks[1]);
        frame.render_widget(c, horizontal_chunks[2]);
        frame.render_widget(d, horizontal_chunks[3]);

        // frame.render_widget(todo!(), chunks[0]);
        // Paragraph::new("hello world"), area);
        Ok(())
    }
}
