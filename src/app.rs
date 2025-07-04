use crate::github::GitHubClient;
use crate::models::Repository;
use ratatui::crossterm::event::KeyCode;
use std::collections::HashMap;
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
    /// Organizations list fetching started
    OrganizationsFetchStarted,
    /// Organizations list fetched
    OrganizationsFetched { organizations: Vec<String> },
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

    /// Current repository view mode (Personal/Organizations)
    pub repo_view_mode: RepositoryViewMode,

    /// List of organizations the user belongs to
    pub user_organizations: Vec<String>,

    /// Current organization index for cycling (0 = Personal, 1+ = organizations)
    pub current_org_index: usize,

    /// Cached personal repositories
    pub personal_repositories: Option<Vec<Repository>>,

    /// Cached organization repositories (org_name -> repositories)
    pub organization_repositories: std::collections::HashMap<String, Vec<Repository>>,

    /// Loading state for async operations
    pub is_loading: bool,

    /// Enhancement state - true when enhancing repositories with details
    pub is_enhancing: bool,

    /// Organizations fetching state - true when fetching organizations
    pub is_fetching_organizations: bool,

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

    /// Repository view mode - personal or organizations
    pub repository_view_mode: RepositoryViewMode,
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

/// Different repository view modes
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryViewMode {
    /// Show user's personal repositories
    Personal,
    /// Show repositories from a specific organization
    Organization(String), // Organization name
}

impl RepositoryViewMode {
    pub fn display_name(&self) -> String {
        match self {
            RepositoryViewMode::Personal => "Personal".to_string(),
            RepositoryViewMode::Organization(org) => format!("Org: {}", org),
        }
    }
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
            repo_view_mode: RepositoryViewMode::Personal,
            user_organizations: Vec::new(),
            current_org_index: 0,
            personal_repositories: None,
            organization_repositories: HashMap::new(),
            is_loading: false,
            is_enhancing: false,
            is_fetching_organizations: false,
            loading_progress: None,
            enhancement_progress: None,
            error_message,
            selected_repository: 0,
            scroll_offset: 0,
            background_receiver: None,
            repository_view_mode: RepositoryViewMode::Personal,
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

            // Tab - cycle between view modes
            KeyCode::Tab => {
                self.cycle_view_mode();
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

        // Clear cache for current mode to force refresh
        match &self.repo_view_mode {
            RepositoryViewMode::Personal => self.personal_repositories = None,
            RepositoryViewMode::Organization(org_name) => {
                self.organization_repositories.remove(org_name);
            }
        }

        // Fetch repositories for current mode
        self.fetch_repositories_for_current_mode();
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

    /// Check if organizations are currently being fetched
    pub fn is_fetching_organizations(&self) -> bool {
        self.is_fetching_organizations
    }

    /// Get the number of available organizations
    pub fn organization_count(&self) -> usize {
        self.user_organizations.len()
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
                        self.repositories = repositories.clone();
                        // Cache the repositories based on current mode
                        match &self.repo_view_mode {
                            RepositoryViewMode::Personal => {
                                self.personal_repositories = Some(repositories);
                            }
                            RepositoryViewMode::Organization(org_name) => {
                                self.organization_repositories
                                    .insert(org_name.clone(), repositories);
                            }
                        }
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
                    BackgroundMessage::OrganizationsFetchStarted => {
                        self.is_fetching_organizations = true;
                    }
                    BackgroundMessage::OrganizationsFetched { organizations } => {
                        self.user_organizations = organizations;
                        self.is_fetching_organizations = false;
                        self.error_message = None;
                        
                        // If the user was trying to cycle but we had no organizations,
                        // now we can start cycling
                        if !self.user_organizations.is_empty() {
                            // The user will need to press Tab again to start cycling
                            // This is more predictable than auto-cycling
                        }
                    }
                }
            }
        }
    }

    /// Cycle between repository view modes
    pub fn cycle_view_mode(&mut self) {
        // If we don't have organizations yet and have a GitHub client, try to fetch them first
        if self.user_organizations.is_empty() && self.github_client.is_some() {
            self.fetch_user_organizations();
            // If we still have no organizations, there's nothing to cycle to
            // so just stay in Personal mode for now
            return;
        }

        // If no organizations available (either no GitHub client or no orgs found), 
        // just stay in Personal mode
        if self.user_organizations.is_empty() {
            return;
        }

        // Cycle through: Personal -> Org1 -> Org2 -> ... -> Personal
        let total_modes = 1 + self.user_organizations.len(); // 1 for Personal + N orgs
        self.current_org_index = (self.current_org_index + 1) % total_modes;

        // Update the view mode based on current index
        self.repo_view_mode = if self.current_org_index == 0 {
            RepositoryViewMode::Personal
        } else {
            let org_name = self.user_organizations[self.current_org_index - 1].clone();
            RepositoryViewMode::Organization(org_name)
        };

        // Switch to the new view
        self.switch_to_current_view();

        // Reset selection and scroll when switching modes
        self.selected_repository = 0;
        self.scroll_offset = 0;
    }

    /// Switch to the current view mode
    fn switch_to_current_view(&mut self) {
        match &self.repo_view_mode {
            RepositoryViewMode::Personal => {
                if let Some(cached_repos) = &self.personal_repositories {
                    // Use cached data
                    self.repositories = cached_repos.clone();
                } else {
                    // Need to fetch personal repositories
                    self.fetch_repositories_for_current_mode();
                }
            }
            RepositoryViewMode::Organization(org_name) => {
                if let Some(cached_repos) = self.organization_repositories.get(org_name) {
                    // Use cached data
                    self.repositories = cached_repos.clone();
                } else {
                    // Need to fetch organization repositories
                    self.fetch_repositories_for_current_mode();
                }
            }
        }
    }

    /// Fetch the list of organizations the user belongs to
    fn fetch_user_organizations(&mut self) {
        if let Some(client) = self.github_client.clone() {
            let sender = self.setup_background_processing();
            
            // Send start message immediately
            let _ = sender.send(BackgroundMessage::OrganizationsFetchStarted);
            
            tokio::spawn(async move {
                match client.get_user_organizations().await {
                    Ok(orgs) => {
                        // Send a message to update the organizations list
                        let _ = sender.send(BackgroundMessage::OrganizationsFetched { organizations: orgs });
                    }
                    Err(e) => {
                        let _ = sender.send(BackgroundMessage::FetchError { 
                            error: format!("Failed to fetch organizations: {}", e) 
                        });
                    }
                }
            });
        }
    }

    /// Fetch repositories for the current view mode
    fn fetch_repositories_for_current_mode(&mut self) {
        if let Some(client) = self.github_client.clone() {
            // Clear current repositories and show loading
            self.repositories.clear();
            self.is_loading = true;
            self.error_message = None;
            self.loading_progress = None;

            // Setup background processing channel
            let sender = self.setup_background_processing();

            // Spawn background task based on current mode
            match &self.repo_view_mode {
                RepositoryViewMode::Personal => {
                    crate::github::GitHubClient::spawn_background_fetch(client, sender);
                }
                RepositoryViewMode::Organization(org_name) => {
                    crate::github::GitHubClient::spawn_background_fetch_organization(
                        client,
                        sender,
                        org_name.clone(),
                    );
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
