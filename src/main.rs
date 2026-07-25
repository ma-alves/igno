mod app;
mod client;
mod handlers;
mod message;
mod update;
mod view;

use color_eyre::Result;
use ratatui::crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use app::{App, RequestStatus};
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

    while app.running {
        terminal.draw(|f| view::view(f, &app))?;

        while let Ok(msg) = rx.try_recv() {
            let mut current = Some(msg);
            while let Some(msg) = current {
                current = update(&mut app, msg);
            }
        }

        if let Some(msg) = handlers::handle_event(&app)? {
            let is_send = matches!(msg, Message::SendRequest);
            let mut current = Some(msg);
            while let Some(msg) = current {
                current = update(&mut app, msg);
            }
            if is_send && app.status == RequestStatus::Loading && !app.pending {
                app.pending = true;
                let tx = tx.clone();
                let url = app.url.clone();
                tokio::spawn(async move {
                    tx.send(Message::ResponseReceived(client::fetch(&url).await))
                        .await
                        .ok();
                });
            }
        }
    }

    Ok(())
}
