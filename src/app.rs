use crate::github::GitHubClient;
use crate::models::Repository;
use ratatui::crossterm::event::KeyCode;
use std::time::Instant;
use tokio::sync::mpsc;

/// Messages sent from background tasks to the main UI thread
#[derive(Debug)]
pub enum BackgroundMessage {
    /// Repository fetching started with total count
    FetchStarted { total: usize },
    /// A single repository was fetched
    RepositoryFetched {
        repository: Repository,
        current: usize,
        total: usize,
    },
    /// All repositories have been fetched (basic info only)
    FetchCompleted { repositories: Vec<Repository> },
    /// An error occurred during fetching
    FetchError { error: String },
    /// Enhancement phase started (additional details)
    EnhancementStarted { total: usize },
    /// A repository was enhanced with additional details
    RepositoryEnhanced {
        repository: Repository,
        current: usize,
        total: usize,
    },
    /// All repositories have been enhanced with full details
    EnhancementCompleted { repositories: Vec<Repository> },
}

/// Application state and configuration
///
/// This struct holds the current state of the application, including
/// UI state, data, and configuration. As we add features, this will
/// grow to include repository data, refresh timers, etc.
#[derive(Debug)]
pub struct App {
    /// Whether the application should quit
    pub should_quit: bool,

    /// Current view/tab (future: could be Dashboard, Settings, etc.)
    pub current_view: AppView,

    /// Application title displayed in the UI
    pub title: String,

    /// Last refresh timestamp (future feature)
    pub last_refresh: Option<std::time::Instant>,

    /// GitHub client for API interactions
    pub github_client: Option<GitHubClient>,

    /// Repository data fetched from GitHub
    pub repositories: Vec<Repository>,

    /// Loading state for async operations
    pub is_loading: bool,

    /// Enhancement state - true when enhancing repositories with details
    pub is_enhancing: bool,

    /// Loading progress information
    pub loading_progress: Option<(usize, usize)>, // (current, total)

    /// Enhancement progress information
    pub enhancement_progress: Option<(usize, usize)>, // (current, total)

    /// Error message if something goes wrong
    pub error_message: Option<String>,

    /// Currently selected repository index
    pub selected_repository: usize,

    /// Scroll position for the repository list
    pub scroll_offset: usize,

    /// Receiver for background task messages
    pub background_receiver: Option<mpsc::UnboundedReceiver<BackgroundMessage>>,
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
        // Try to initialize GitHub client
        let (github_client, error_message) = match GitHubClient::new() {
            Ok(client) => (Some(client), None),
            Err(e) => (None, Some(format!("GitHub setup error: {}", e))),
        };

        Self {
            should_quit: false,
            current_view: AppView::Dashboard,
            title: "❤️ Repo Health Dashboard ❤️".to_string(),
            last_refresh: None,
            github_client,
            repositories: Vec::new(),
            is_loading: false,
            is_enhancing: false,
            loading_progress: None,
            enhancement_progress: None,
            error_message,
            selected_repository: 0,
            scroll_offset: 0,
            background_receiver: None,
        }
    }

    /// Initialize the GitHub client
    pub fn initialize_github_client(&mut self) {
        match GitHubClient::new() {
            Ok(client) => {
                self.github_client = Some(client);
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to initialize GitHub client: {}", e));
            }
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

            // Navigation - Up arrow
            KeyCode::Up => {
                if !self.repositories.is_empty() {
                    if self.selected_repository > 0 {
                        self.selected_repository -= 1;
                        // Auto-scroll to keep selection visible
                        self.ensure_selected_visible(10); // Assume at least 10 items visible
                    }
                }
                true
            }

            // Navigation - Down arrow
            KeyCode::Down => {
                if !self.repositories.is_empty() {
                    if self.selected_repository < self.repositories.len() - 1 {
                        self.selected_repository += 1;
                        // Auto-scroll to keep selection visible
                        self.ensure_selected_visible(10); // Assume at least 10 items visible
                    }
                }
                true
            }

            // Page Up - scroll up by page
            KeyCode::PageUp => {
                if !self.repositories.is_empty() {
                    // Move selection up by 10 items or to the top
                    if self.selected_repository >= 10 {
                        self.selected_repository -= 10;
                    } else {
                        self.selected_repository = 0;
                    }
                    // Update scroll position
                    self.ensure_selected_visible(10);
                }
                true
            }

            // Page Down - scroll down by page
            KeyCode::PageDown => {
                if !self.repositories.is_empty() {
                    // Move selection down by 10 items or to the bottom
                    if self.selected_repository + 10 < self.repositories.len() {
                        self.selected_repository += 10;
                    } else {
                        self.selected_repository = self.repositories.len() - 1;
                    }
                    // Update scroll position
                    self.ensure_selected_visible(10);
                }
                true
            }

            // Home - jump to top
            KeyCode::Home => {
                if !self.repositories.is_empty() {
                    self.selected_repository = 0;
                    self.scroll_offset = 0;
                }
                true
            }

            // End - jump to bottom
            KeyCode::End => {
                if !self.repositories.is_empty() {
                    self.selected_repository = self.repositories.len() - 1;
                    // Let ensure_selected_visible handle the scroll
                    self.ensure_selected_visible(10);
                }
                true
            }

            // Future key handlers:
            // KeyCode::Tab => self.next_view(),
            // KeyCode::BackTab => self.previous_view(),
            _ => false, // Event not handled
        }
    }

    /// Refresh application data
    ///
    /// Updates the last refresh timestamp and initializes GitHub client if needed.
    /// Marks the app as loading to trigger data fetching.
    pub fn refresh(&mut self) {
        self.last_refresh = Some(std::time::Instant::now());

        // If no GitHub client is initialized, try to initialize it
        if self.github_client.is_none() {
            self.initialize_github_client();
        }

        // Start background fetching if client is available
        if let Some(client) = self.github_client.clone() {
            // Clear any existing state
            self.is_loading = true;
            self.error_message = None;
            self.repositories.clear();
            self.loading_progress = None;

            // Setup background processing channel
            let sender = self.setup_background_processing();

            // Spawn background task
            crate::github::GitHubClient::spawn_background_fetch(client, sender);
        }
    }

    /// Async method to fetch repository data from GitHub
    /// This should be called from the main event loop when is_loading is true
    pub async fn fetch_repositories(&mut self) -> Result<(), String> {
        if let Some(ref client) = self.github_client {
            self.is_loading = true;
            self.error_message = None;

            match client.list_user_repositories().await {
                Ok(repos) => {
                    self.repositories = repos;
                    self.is_loading = false;
                    self.last_refresh = Some(std::time::Instant::now());
                    Ok(())
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to fetch repositories: {}", e));
                    self.is_loading = false;
                    Err(e)
                }
            }
        } else {
            // GitHub client not available - this should have been set during app initialization
            // If we get here, it means the token wasn't set or GitHub client creation failed
            if self.error_message.is_none() {
                self.error_message = Some("GitHub client not available. Please check your GH_REPO_HEALTHCHECKS_TOKEN environment variable.".to_string());
            }
            self.is_loading = false;
            Err("GitHub client not initialized".into())
        }
    }

    /// Get the display title for the current view
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the current repositories
    pub fn get_repositories(&self) -> &[Repository] {
        &self.repositories
    }

    /// Check if the app is currently loading data
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// Check if repositories are being enhanced with additional details
    pub fn is_enhancing(&self) -> bool {
        self.is_enhancing
    }

    /// Get the enhancement progress, if available
    pub fn enhancement_progress(&self) -> Option<(usize, usize)> {
        self.enhancement_progress
    }

    /// Get the current error message, if any
    pub fn get_error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Clear the current error message
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Get the number of repositories currently loaded
    pub fn repository_count(&self) -> usize {
        self.repositories.len()
    }

    /// Get the currently selected repository index
    pub fn selected_repository(&self) -> usize {
        self.selected_repository
    }

    /// Move selection to the next repository
    pub fn select_next_repository(&mut self) {
        if !self.repositories.is_empty() {
            self.selected_repository = (self.selected_repository + 1) % self.repositories.len();
        }
    }

    /// Move selection to the previous repository  
    pub fn select_previous_repository(&mut self) {
        if !self.repositories.is_empty() {
            if self.selected_repository == 0 {
                self.selected_repository = self.repositories.len() - 1;
            } else {
                self.selected_repository -= 1;
            }
        }
    }

    /// Get a title with repository stats
    pub fn get_title_with_stats(&self) -> String {
        if self.repositories.is_empty() && !self.is_loading {
            format!("{} — No repositories found", self.title)
        } else if self.is_loading {
            format!("{} — Loading...", self.title)
        } else {
            let active_count = self
                .repositories
                .iter()
                .filter(|repo| !repo.open_pull_requests.is_empty())
                .count();

            format!(
                "{} — {} repos ({} active)",
                self.title,
                self.repositories.len(),
                active_count
            )
        }
    }

    /// Update loading progress
    pub fn set_loading_progress(&mut self, current: usize, total: usize) {
        self.loading_progress = Some((current, total));
    }

    /// Clear loading progress
    pub fn clear_loading_progress(&mut self) {
        self.loading_progress = None;
    }

    /// Get visible items count based on the area height
    pub fn get_visible_item_count(&self, area_height: usize) -> usize {
        // Account for header row
        if area_height > 1 {
            area_height - 1
        } else {
            0
        }
    }

    /// Scroll up in the repository list
    pub fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    /// Scroll down in the repository list
    pub fn scroll_down(&mut self, visible_items: usize) {
        let max_scroll = self.repositories.len().saturating_sub(visible_items);
        if self.scroll_offset < max_scroll {
            self.scroll_offset += 1;
        }
    }

    /// Ensure the selected repository is visible
    pub fn ensure_selected_visible(&mut self, visible_items: usize) {
        // If selected is above viewport, scroll up
        if self.selected_repository < self.scroll_offset {
            self.scroll_offset = self.selected_repository;
        }

        // If selected is below viewport, scroll down
        if self.selected_repository >= self.scroll_offset + visible_items {
            self.scroll_offset = self.selected_repository.saturating_sub(visible_items) + 1;
        }
    }

    /// Set up background task processing
    pub fn setup_background_processing(&mut self) -> mpsc::UnboundedSender<BackgroundMessage> {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.background_receiver = Some(receiver);
        sender
    }

    /// Process any pending background messages
    pub fn process_background_messages(&mut self) {
        if let Some(receiver) = &mut self.background_receiver {
            while let Ok(message) = receiver.try_recv() {
                match message {
                    BackgroundMessage::FetchStarted { total } => {
                        self.is_loading = true;
                        self.loading_progress = Some((0, total));
                        self.error_message = None;
                    }
                    BackgroundMessage::RepositoryFetched {
                        repository,
                        current,
                        total,
                    } => {
                        self.repositories.push(repository);
                        self.loading_progress = Some((current, total));
                    }
                    BackgroundMessage::FetchCompleted { repositories } => {
                        self.repositories = repositories;
                        // We've loaded basic data, but will start enhancing
                        self.is_loading = false;
                        self.loading_progress = None;
                        self.last_refresh = Some(std::time::Instant::now());
                    }
                    BackgroundMessage::FetchError { error } => {
                        self.error_message = Some(error);
                        self.is_loading = false;
                        self.is_enhancing = false;
                        self.loading_progress = None;
                        self.enhancement_progress = None;
                    }
                    BackgroundMessage::EnhancementStarted { total } => {
                        // We already have basic data and are now enhancing
                        self.is_enhancing = true;
                        self.enhancement_progress = Some((0, total));
                    }
                    BackgroundMessage::RepositoryEnhanced {
                        repository,
                        current,
                        total,
                    } => {
                        // Find and replace the repository with the enhanced version
                        if let Some(index) = self
                            .repositories
                            .iter()
                            .position(|r| r.name == repository.name)
                        {
                            self.repositories[index] = repository;
                        }
                        self.enhancement_progress = Some((current, total));
                    }
                    BackgroundMessage::EnhancementCompleted { repositories } => {
                        self.repositories = repositories;
                        self.is_enhancing = false;
                        self.enhancement_progress = None;
                        self.last_refresh = Some(std::time::Instant::now());
                    }
                }
            }
        }
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
