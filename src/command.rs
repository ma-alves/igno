use tokio::sync::mpsc;

use crate::client;
use crate::message::Message;

pub enum Command {
    None,
    FetchUrl(String),
}

pub fn command_handler(cmd: Command, tx: mpsc::Sender<Message>) {
    match cmd {
        Command::None => {}
        Command::FetchUrl(url) => {
            tokio::spawn(async move {
                let result = client::fetch(&url).await;
                let _ = tx.send(Message::ResponseReceived(result)).await;
            });
        }
    }
}
