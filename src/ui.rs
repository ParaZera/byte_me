use crate::app::{App, DataType, Endianness, InputFormat};
use ratatui::{
    Frame,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

pub fn draw<B: Backend>(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Input line
                Constraint::Min(10),   // Conversion results
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_input_line::<B>(f, app, chunks[0]);
    draw_results::<B>(f, app, chunks[1]);

    // Draw dropdown if open
    if app.dropdown_open {
        let popup_area = centered_rect(30, 40, f.area());
        match app.active_dropdown {
            0 => draw_data_type_dropdown::<B>(f, app, popup_area),
            1 => draw_endianness_dropdown::<B>(f, app, popup_area),
            2 => draw_input_format_dropdown::<B>(f, app, popup_area),
            _ => unreachable!(),
        }
    }
}

fn draw_input_line<B: Backend>(f: &mut Frame, app: &App, area: Rect) {
    let data_type_style = if app.active_dropdown == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let endianness_style = if app.active_dropdown == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let format_style = if app.active_dropdown == 2 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let input_style = if app.input_mode && !app.dropdown_open {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    // Create the input line with dropdown selections and number input
    let input_segments = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(8), // Data type dropdown
                Constraint::Length(1), // Separator
                Constraint::Length(8), // Endianness dropdown
                Constraint::Length(1), // Separator
                Constraint::Length(8), // Format dropdown
                Constraint::Length(1), // Separator
                Constraint::Min(10),   // Number input
            ]
            .as_ref(),
        )
        .split(area);

    // Data type dropdown
    let data_type_text = format!("[ {} ]", app.data_type.to_short_string());
    let data_type_widget = Paragraph::new(data_type_text)
        .style(data_type_style)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(data_type_widget, input_segments[0]);

    // Endianness dropdown
    let endianness_text = format!("[ {} ]", app.endianness.to_short_string());
    let endianness_widget = Paragraph::new(endianness_text)
        .style(endianness_style)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(endianness_widget, input_segments[2]);

    // Format dropdown
    let format_text = format!("[ {} ]", app.input_format.to_short_string());
    let format_widget = Paragraph::new(format_text)
        .style(format_style)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(format_widget, input_segments[4]);

    // Number input with prefix
    let prefix = app.input_format.number_prefix();

    let input_text = format!("{}{}", prefix, app.input);
    let input_widget = Paragraph::new(input_text)
        .style(input_style)
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(input_widget, input_segments[6]);
}

fn draw_results<B: Backend>(f: &mut Frame, app: &App, area: Rect) {
    let results = app.get_conversion_results();

    let items: Vec<ListItem> = results
        .iter()
        .map(|(label, value)| {
            ListItem::new(Line::from(vec![
                Span::styled(format!("{}: ", label), Style::default().fg(Color::Blue)),
                Span::raw(value),
            ]))
        })
        .collect();

    let results_widget = List::new(items)
        .block(Block::default().title("Conversions").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(results_widget, area);
}

fn draw_data_type_dropdown<B: Backend>(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = DataType::all()
        .iter()
        .map(|dt| {
            let style = if *dt == app.data_type {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(dt.to_string()).style(style)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.dropdown_index));

    let list = List::new(items)
        .block(Block::default().title("Data Type").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(Clear, area);
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_endianness_dropdown<B: Backend>(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = Endianness::all()
        .iter()
        .map(|e| {
            let style = if *e == app.endianness {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(e.to_string()).style(style)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.dropdown_index));

    let list = List::new(items)
        .block(Block::default().title("Endianness").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(Clear, area);
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_input_format_dropdown<B: Backend>(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = InputFormat::all()
        .iter()
        .map(|fmt| {
            let style = if *fmt == app.input_format {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(fmt.to_string()).style(style)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.dropdown_index));

    let list = List::new(items)
        .block(Block::default().title("Input Format").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(Clear, area);
    f.render_stateful_widget(list, area, &mut state);
}

// Helper function to create a centered rect using up certain percentage of the available rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
