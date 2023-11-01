use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::{Frame, Terminal};

mod error;

use error::CounterError;

const EVENT_POLL_TIME: u64 = 250;

struct AppState {
    counter: i64,
    should_quit: bool,
}

fn main() -> anyhow::Result<()> {
    startup()?;
    let status = run();
    shutdown()?;
    status?;

    Ok(())
}

fn startup() -> Result<(), CounterError> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}
fn shutdown() -> Result<(), CounterError> {
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn ui(app: &AppState, f: &mut Frame<'_>) {
    f.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter)),
        f.size(),
    );
}

fn update(app: &mut AppState) -> Result<(), CounterError> {
    if crossterm::event::poll(std::time::Duration::from_millis(EVENT_POLL_TIME))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                match key.code {
                    crossterm::event::KeyCode::Char('j') => app.counter += 1,
                    crossterm::event::KeyCode::Char('k') => app.counter -= 1,
                    crossterm::event::KeyCode::Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<(), CounterError> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let mut app = AppState {
        counter: 0,
        should_quit: false,
    };

    loop {
        // Draw a frame
        terminal.draw(|f| {
            ui(&app, f);
        })?;

        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
