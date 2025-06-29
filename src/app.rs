use ratatui::crossterm::event::KeyCode;

/// Application state and configuration
///
/// This struct holds the current state of the application, including
/// UI state, data, and configuration. As we add features, this will
/// grow to include repository data, refresh timers, etc.
#[derive(Debug, Default)]
pub struct App {
    /// Whether the application should quit
    pub should_quit: bool,

    /// Current view/tab (future: could be Dashboard, Settings, etc.)
    pub current_view: AppView,

    /// Application title displayed in the UI
    pub title: String,

    /// Last refresh timestamp (future feature)
    pub last_refresh: Option<std::time::Instant>,
}

/// Different views/screens in the application
#[derive(Debug, Default, Clone, PartialEq)]
pub enum AppView {
    #[default]
    Dashboard,
    // Future views:
    // Settings,
    // RepoDetails,
    // Help,
}

impl App {
    /// Create a new application instance with default settings
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_view: AppView::Dashboard,
            title: "🛡️ Team Repo Health Dashboard".to_string(),
            last_refresh: None,
        }
    }

    /// Handle keyboard input and update application state
    ///
    /// Returns true if the event was handled, false otherwise
    pub fn handle_key_event(&mut self, key_code: KeyCode) -> bool {
        match key_code {
            // Quit the application
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                self.should_quit = true;
                true
            }

            // Refresh data (future implementation)
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::F(5) => {
                self.refresh();
                true
            }

            // Future key handlers:
            // KeyCode::Tab => self.next_view(),
            // KeyCode::BackTab => self.previous_view(),
            // KeyCode::Up => self.previous_item(),
            // KeyCode::Down => self.next_item(),
            _ => false, // Event not handled
        }
    }

    /// Refresh application data
    ///
    /// Currently just updates the last refresh timestamp.
    /// Future: This will trigger GitHub API calls to fetch fresh data.
    pub fn refresh(&mut self) {
        self.last_refresh = Some(std::time::Instant::now());
        // Future: Trigger data refresh from GitHub API
    }

    /// Get the display title for the current view
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(!app.should_quit());
        assert_eq!(app.current_view, AppView::Dashboard);
    }

    #[test]
    fn test_quit_key_handling() {
        let mut app = App::new();

        // Test 'q' key
        assert!(app.handle_key_event(KeyCode::Char('q')));
        assert!(app.should_quit());
    }

    #[test]
    fn test_refresh_key_handling() {
        let mut app = App::new();

        // Test 'r' key
        assert!(app.handle_key_event(KeyCode::Char('r')));
        assert!(app.last_refresh.is_some());
    }
}
