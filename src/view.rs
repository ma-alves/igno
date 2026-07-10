use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{App, RequestStatus};

/// Pure: given the app, draw the frame. No mutation, no I/O.
/// This is the "V" in App-Update-View — it never sends a `Message`
/// or reaches into `Command`; it only reads.
pub fn view(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // URL bar
            Constraint::Length(3), // status line
            Constraint::Min(0),    // response body
            Constraint::Length(1), // help footer
        ])
        .split(frame.area());

    draw_url_bar(frame, app, chunks[0]);
    draw_status(frame, app, chunks[1]);
    draw_response(frame, app, chunks[2]);
    draw_footer(frame, chunks[3]);
}

fn draw_url_bar(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().title("URL (GET)").borders(Borders::ALL);
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
    let block = Block::default()
        .title("Response Body")
        .borders(Borders::ALL);
    let text = if let Some(resp) = &app.response {
        Paragraph::new(resp.body.as_str())
            .block(block)
            .wrap(Wrap { trim: false })
    } else {
        Paragraph::new("").block(block)
    };
    frame.render_widget(text, area);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new("Enter: send request  |  Backspace: edit URL  |  q: quit")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(text, area);
}
