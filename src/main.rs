mod app;
mod events;
mod github;
mod models;
mod terminal;
mod ui;

use app::App;
use events::EventHandler;
use terminal::TerminalManager;
use ui::UI;

use std::error::Error;

/// Main entry point for the GitHub Repository Health Dashboard
///
/// This function sets up the terminal environment, initializes the TUI,
/// runs the main application loop, and handles cleanup when exiting.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the terminal and run the app
    let mut terminal = TerminalManager::setup()?;
    let result = run_app(&mut terminal).await;

    // Clean up terminal state before exiting
    TerminalManager::cleanup(&mut terminal)?;

    // Report any errors that occurred during execution
    if let Err(err) = result {
        eprintln!("Application error: {:?}", err);
    }

    Ok(())
}

/// Main application event loop
///
/// This function handles:
/// - Managing application state
/// - Processing events (keyboard input, terminal resize, etc.)
/// - Rendering the UI on each frame
/// - Graceful exit when requested
async fn run_app(
    terminal: &mut ratatui::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    // Initialize application state
    let mut app = App::new();
    let event_handler = EventHandler::new();

    // Trigger initial refresh to start background loading
    app.refresh();

    // Main event loop
    loop {
        // Process any pending background messages
        app.process_background_messages();

        // Draw the current frame
        terminal.draw(|frame| {
            UI::render(frame, &app);
        })?;

        // Check for and handle events with a timeout to allow async operations
        if let Some(event) = event_handler.next_event()? {
            // Handle the event based on its type
            match event {
                events::AppEvent::Key(key_event) => {
                    if event.is_refresh() {
                        // Refresh repositories in the background
                        app.refresh();
                    } else {
                        // Let the app handle other key events
                        app.handle_key_event(key_event.code);
                    }
                }
                events::AppEvent::Resize(_width, _height) => {
                    // Terminal was resized - the next draw will handle the new size
                    // No explicit action needed as ratatui handles this automatically
                }
                events::AppEvent::Mouse(_mouse_event) => {
                    // Future: Handle mouse events for enhanced interaction
                }
            }
        }

        // Check if the application should quit
        if app.should_quit() {
            break;
        }

        // Small sleep to prevent 100% CPU usage
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    Ok(())
}
