use crate::client::Response;

#[derive(Debug)]
pub enum Message {
    Char(char),
    Backspace,
    Quit,
    SendRequest,
    ResponseReceived(Result<Response, String>),
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    ToggleFocus,
}
