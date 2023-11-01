mod app;
mod error;
mod event;
mod tui;
mod ui;
mod update;

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use app::AppState;
use event::{Event, EventHandler};
use tui::Tui;
use update::update;

const EVENT_POLL_TIME: u64 = 250;

fn main() -> anyhow::Result<()> {
    let mut app = AppState::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(EVENT_POLL_TIME);

    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit() {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
