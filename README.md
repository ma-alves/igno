# igno

**igno** is a terminal-based HTTP client — a TUI alternative to those Electron tools, built with [Ratatui](https://ratatui.rs) and [reqwest](https://docs.rs/reqwest).

## Features I'll be working on according to the AI Agent

- Send HTTP requests (GET, POST, PUT, PATCH, DELETE) to any URL
- View response status, headers, and body directly in your terminal
- Lightweight and keyboard-driven

## Architecture the AI Agent assumed (it's wrong)

igno follows the [Elm architecture](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/):

- `app.rs` — application state (model)
- `message.rs` — event definitions
- `update.rs` — pure `(Message, &mut App) -> Command`
- `command.rs` — side-effect descriptions
- `client.rs` — HTTP client (only module with async I/O)
- `view.rs` — pure rendering
- `main.rs` — runtime and event loop

## To-Do
We're having a to-do section here!!!

## License

MIT
