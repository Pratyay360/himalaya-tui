use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::app::{App, Panel};
use super::get_border_style;

pub fn render_envelopes(frame: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["UID", "Date", "From", "Subject"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
    let header = Row::new(header_cells)
        .style(Style::default().fg(Color::Yellow))
        .height(1);

    let rows: Vec<Row> = app
        .envelopes
        .iter()
        .enumerate()
        .map(|(i, envelope)| {
            let style = if i == app.envelope_index && app.active_panel == Panel::Envelopes {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else if envelope.flags.iter().any(|f| f == "\\Seen") {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().add_modifier(Modifier::BOLD)
            };

            let cells = vec![
                Cell::from(envelope.uid.to_string()),
                Cell::from(envelope.date.clone()),
                Cell::from(truncate(&envelope.from, 20)),
                Cell::from(truncate(&envelope.subject, 40)),
            ];

            Row::new(cells).style(style)
        })
        .collect();

    let block = Block::default()
        .title(format!(
            " Envelopes{} ",
            app.selected_mailbox
                .as_ref()
                .map(|m| format!(" - {}", m))
                .unwrap_or_default()
        ))
        .borders(Borders::ALL)
        .border_style(get_border_style(app, Panel::Envelopes));

    let widths = [
        Constraint::Length(8),
        Constraint::Length(12),
        Constraint::Length(22),
        Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(table, area);
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max_len - 3).collect::<String>())
    }
}
