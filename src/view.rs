use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, RequestStatus};

pub fn view(frame: &mut Frame, app: &App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // status
            Constraint::Min(0),    // request | response
            Constraint::Length(1), // footer
        ])
        .split(frame.area());

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // request (left)
            Constraint::Percentage(50), // response (right)
        ])
        .split(vertical[1]);

    draw_status(frame, app, vertical[0]);
    draw_request(frame, app, horizontal[0]);
    draw_response(frame, app, horizontal[1]);
    draw_footer(frame, vertical[2]);
}

fn draw_request(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title("Request").borders(Borders::ALL);
    let text = Paragraph::new(app.url.as_str()).block(block);
    frame.render_widget(text, area);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let (label, color) = match app.status {
        RequestStatus::Loading => ("Loading...".to_string(), Color::Yellow),
        RequestStatus::Idle => match (&app.response, &app.error) {
            (_, Some(e)) => (format!("Error: {e}"), Color::Red),
            (Some(resp), None) => match resp.status {
                Some(code) => (format!("Status: {code}"), Color::Green),
                None => ("Idle".to_string(), Color::DarkGray),
            },
            (None, None) => ("Idle".to_string(), Color::DarkGray),
        },
    };
    let block = Block::default().title("Status").borders(Borders::ALL);
    let text = Paragraph::new(Line::from(Span::styled(
        label,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )))
    .block(block);
    frame.render_widget(text, area);
}

fn draw_response(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title("Response").borders(Borders::ALL);
    let response_text = if let Some(resp) = &app.response {
        Paragraph::new(format!(
            "Status: {}\nDuration: {} ms\nHeaders:\n{}\n\nBody:\n{}",
            resp.status
                .map(|s| s.to_string())
                .unwrap_or_else(|| "<unknown>".to_string()),
            resp.duration,
            resp.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n"),
            resp.body
        ))
        .block(block)
    } else {
        Paragraph::new("").block(block)
    };
    frame.render_widget(response_text, area);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new("Send: enter  |  Pop: backspace  |  Quit: q")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(text, area);
}
