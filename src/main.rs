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

use app::App;
use message::Message;
use update::Command;

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

fn process_update(app: &mut App, msg: Message, tx: &mpsc::Sender<Message>) {
    let cmd = update::update(app, msg);
    if let Command::Fetch { url } = cmd {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(Message::ResponseReceived(client::fetch(&url).await))
                .await
                .ok();
        });
    }
}

async fn run(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    let mut app = App::default();
    let (tx, mut rx) = mpsc::channel::<Message>(32);
    let mut should_draw = true;

    while app.running {
        if should_draw {
            terminal.draw(|f| view::view(f, &app))?;
            should_draw = false;
        }

        while let Ok(msg) = rx.try_recv() {
            process_update(&mut app, msg, &tx);
            should_draw = true;
        }

        if let Some(msg) = handlers::handle_event(&app)? {
            process_update(&mut app, msg, &tx);
            should_draw = true;
        }
    }

    Ok(())
}
