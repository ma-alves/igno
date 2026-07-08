use crate::client::ApiResult;
use ratatui::crossterm::event::KeyEvent;

/// Every single thing that can happen in this app, named as data.
#[derive(Debug)]
pub enum Message {
    KeyPressed(KeyEvent),
    SendRequest,
    ResponseReceived(Result<ApiResult, String>),
    Quit,
}