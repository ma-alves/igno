mod client;
mod command;
mod app;
mod message;
mod update;
mod view;

use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::{
    event::{self as cevent, Event as CEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use command::Command;
use app::App;
use message::Message;
use update::update;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut app = App::default();
    let (tx, mut rx) = mpsc::channel::<Message>(32);

    loop {
        terminal.draw(|f| view::view(f, &app))?;

        // Any Message that arrived asynchronously (e.g. an API response).
        while let Ok(message) = rx.try_recv() {
            let effect = update(message, &mut app);
            perform(effect, tx.clone());
        }

        // Poll with a short timeout so the loop still wakes up
        // regularly to check the channel above.
        if cevent::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = cevent::read()? {
                let effect = update(Message::KeyPressed(key), &mut app);
                perform(effect, tx.clone());
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// The only place a `Command` turns into a real side effect.
fn perform(cmd: Command, tx: mpsc::Sender<Message>) {
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