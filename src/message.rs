use ratatui::crossterm::event::KeyEvent;

use crate::app::Response;

#[derive(Debug)]
pub enum Message {
    KeyPressed(KeyEvent),
    SendRequest,
    ResponseReceived(Result<Response, String>),
}
