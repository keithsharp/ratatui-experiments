use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

const EVENT_POLL_TIME: u64 = 250;

fn main() -> anyhow::Result<()> {
    // Initialise terminal
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Application state
    let mut counter = 0;

    loop {
        // Draw a frame
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
        })?;

        // Process events
        if crossterm::event::poll(std::time::Duration::from_millis(EVENT_POLL_TIME))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // Reset terminal
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
