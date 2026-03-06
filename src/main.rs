use std::{io, path::PathBuf};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use himalaya_tui::app::App;
use himalaya_tui::ui;

#[cfg(feature = "imap")]
use himalaya_tui::imap;

fn main() -> Result<()> {
    let config_paths = get_config_paths();
    let account_name = std::env::args().nth(1);

    let mut app = App::new(&config_paths, account_name.as_deref())?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    #[cfg(feature = "imap")]
    {
        app.set_status("Connecting to IMAP server...");
        terminal.draw(|f| ui::render(f, &app))?;

        match imap::fetch_mailboxes(&app.imap_config) {
            Ok(mailboxes) => app.set_mailboxes(mailboxes),
            Err(e) => app.set_status(format!("Error: {}", e)),
        }
    }

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while app.running {
        terminal.draw(|f| ui::render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                KeyCode::Tab => app.toggle_panel(),
                KeyCode::Char('j') | KeyCode::Down => app.next_item(),
                KeyCode::Char('k') | KeyCode::Up => app.previous_item(),
                KeyCode::Enter => {
                    #[cfg(feature = "imap")]
                    handle_enter(app);
                }
                KeyCode::Char('r') => {
                    #[cfg(feature = "imap")]
                    refresh_current(app);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

#[cfg(feature = "imap")]
fn handle_enter(app: &mut App) {
    use himalaya_tui::app::Panel;

    match app.active_panel {
        Panel::Mailboxes => {
            app.select_mailbox();
            if let Some(ref mailbox) = app.selected_mailbox {
                match imap::fetch_envelopes(&app.imap_config, mailbox) {
                    Ok(envelopes) => app.set_envelopes(envelopes),
                    Err(e) => app.set_status(format!("Error: {}", e)),
                }
            }
        }
        Panel::Envelopes => {
            // Future: open message view
        }
    }
}

#[cfg(feature = "imap")]
fn refresh_current(app: &mut App) {
    app.set_status("Refreshing...");
    match imap::fetch_mailboxes(&app.imap_config) {
        Ok(mailboxes) => {
            app.set_mailboxes(mailboxes);
            if let Some(ref mailbox) = app.selected_mailbox.clone() {
                match imap::fetch_envelopes(&app.imap_config, mailbox) {
                    Ok(envelopes) => app.set_envelopes(envelopes),
                    Err(e) => app.set_status(format!("Error: {}", e)),
                }
            }
        }
        Err(e) => app.set_status(format!("Error: {}", e)),
    }
}

fn get_config_paths() -> Vec<PathBuf> {
    if let Ok(paths) = std::env::var("HIMALAYA_CONFIG") {
        paths
            .split(':')
            .filter_map(|p| {
                let expanded = shellexpand::full(p).ok()?;
                Some(PathBuf::from(expanded.as_ref()))
            })
            .collect()
    } else {
        Vec::new()
    }
}
