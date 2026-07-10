mod app;
mod client;
mod command;
mod message;
mod update;
mod view;

use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::{
    event::{self as cevent, Event as CEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use app::App;
use command::command_handler;
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

        // Any Message that arrived asynchronously.
        while let Ok(message) = rx.try_recv() {
            let command = update(message, &mut app);
            command_handler(command, tx.clone());
        }

        if cevent::poll(Duration::from_millis(50))? {
            if let CEvent::Key(key) = cevent::read()? {
                let command = update(Message::KeyPressed(key), &mut app);
                command_handler(command, tx.clone());
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
