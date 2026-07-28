# igno

igno is a terminal-based HTTP client. A keyboard-driven TUI alternative to
tools like Postman or Insomnia, built with Ratatui. And my first open source project!

## Features

- Send HTTP requests (GET, POST, PUT, PATCH, DELETE, HEAD) to any URL
- View response status code, headers, and body in your terminal
- Keyboard-driven navigation with Tab to switch focus between request and
  response panels
- Scrollable response body (arrow keys, Page Up / Page Down)

## Architecture

igno follows the Elm-inspired architecture pattern:

- `app.rs` — application state (model)
- `message.rs` — event definitions
- `update.rs` — pure (Message, &mut App) -> Command
- `client.rs` — HTTP client with Client struct, Method enum, and Response type
- `view.rs` — pure rendering
- `main.rs` — runtime and event loop
- `handlers.rs` — keyboard event mapping

## Contributing

Contributions are welcome. Open an issue or submit a pull request.

## License

MIT
