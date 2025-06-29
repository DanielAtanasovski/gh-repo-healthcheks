use std::time::{Duration, SystemTime};

/// Represents the overall health status of a repository
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryStatus {
    /// All checks are passing
    Healthy,
    /// Some warnings but not critical
    Warning,
    /// Critical issues detected
    Critical,
    /// Status is unknown or being fetched
    Unknown,
}

impl RepositoryStatus {
    /// Get a human-readable description of the status
    pub fn description(&self) -> &'static str {
        match self {
            RepositoryStatus::Healthy => "All systems operational",
            RepositoryStatus::Warning => "Some issues detected",
            RepositoryStatus::Critical => "Critical issues require attention",
            RepositoryStatus::Unknown => "Status unknown",
        }
    }
    
    /// Get an emoji representation of the status
    pub fn emoji(&self) -> &'static str {
        match self {
            RepositoryStatus::Healthy => "✅",
            RepositoryStatus::Warning => "⚠️",
            RepositoryStatus::Critical => "❌",
            RepositoryStatus::Unknown => "❓",
        }
    }
}

/// Represents the status of a CI/CD workflow
#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowStatus {
    /// Workflow completed successfully
    Success,
    /// Workflow failed
    Failed,
    /// Workflow is currently running
    InProgress,
    /// Workflow was cancelled
    Cancelled,
    /// Unknown or pending status
    Unknown,
}

impl WorkflowStatus {
    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            WorkflowStatus::Success => "Passed",
            WorkflowStatus::Failed => "Failed",
            WorkflowStatus::InProgress => "Running",
            WorkflowStatus::Cancelled => "Cancelled",
            WorkflowStatus::Unknown => "Unknown",
        }
    }
    
    /// Get an emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            WorkflowStatus::Success => "✅",
            WorkflowStatus::Failed => "❌",
            WorkflowStatus::InProgress => "⏳",
            WorkflowStatus::Cancelled => "🚫",
            WorkflowStatus::Unknown => "❓",
        }
    }
}

/// Represents a GitHub Actions workflow run
#[derive(Debug, Clone)]
pub struct WorkflowRun {
    /// Unique identifier for the workflow run
    pub id: u64,
    /// Name of the workflow
    pub name: String,
    /// Current status of the workflow
    pub status: WorkflowStatus,
    /// When the workflow was created
    pub created_at: SystemTime,
    /// When the workflow was last updated
    pub updated_at: SystemTime,
    /// Duration of the workflow (if completed)
    pub duration: Option<Duration>,
    /// Conclusion of the workflow (if completed)
    pub conclusion: Option<String>,
    /// URL to view the workflow on GitHub
    pub html_url: String,
}

impl WorkflowRun {
    /// Get a human-readable time description for when this run occurred
    pub fn time_description(&self) -> String {
        // Future: Implement relative time formatting
        // For now, just return a placeholder
        "recently".to_string()
    }
    
    /// Check if this workflow run is recent (within last 24 hours)
    pub fn is_recent(&self) -> bool {
        if let Ok(elapsed) = self.created_at.elapsed() {
            elapsed < Duration::from_secs(24 * 60 * 60) // 24 hours in seconds
        } else {
            false
        }
    }
}

/// Represents a pull request
#[derive(Debug, Clone)]
pub struct PullRequest {
    /// Unique identifier for the PR
    pub number: u32,
    /// Title of the pull request
    pub title: String,
    /// Current state (open, closed, merged)
    pub state: PullRequestState,
    /// When the PR was created
    pub created_at: SystemTime,
    /// When the PR was last updated
    pub updated_at: SystemTime,
    /// Author of the pull request
    pub author: String,
    /// URL to view the PR on GitHub
    pub html_url: String,
    /// Whether the PR is a draft
    pub draft: bool,
    /// Number of review approvals
    pub approvals: u32,
    /// Number of requested changes
    pub changes_requested: u32,
}

/// Represents the state of a pull request
#[derive(Debug, Clone, PartialEq)]
pub enum PullRequestState {
    Open,
    Closed,
    Merged,
}

impl PullRequestState {
    /// Get an emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            PullRequestState::Open => "🟢",
            PullRequestState::Closed => "🔴",
            PullRequestState::Merged => "🟣",
        }
    }
}

/// Represents a complete repository with all its health data
#[derive(Debug, Clone)]
pub struct Repository {
    /// Repository name
    pub name: String,
    /// Repository owner/organization
    pub owner: String,
    /// Overall health status
    pub status: RepositoryStatus,
    /// Most recent workflow run
    pub latest_workflow: Option<WorkflowRun>,
    /// Open pull requests
    pub open_pull_requests: Vec<PullRequest>,
    /// When this data was last fetched
    pub last_updated: SystemTime,
    /// URL to the repository on GitHub
    pub html_url: String,
    /// Repository description
    pub description: Option<String>,
    /// Primary programming language
    pub language: Option<String>,
    /// Number of stars
    pub stars: u32,
}

impl Repository {
    /// Create a new repository with default values
    pub fn new(name: String, owner: String) -> Self {
        Self {
            name,
            owner,
            status: RepositoryStatus::Unknown,
            latest_workflow: None,
            open_pull_requests: Vec::new(),
            last_updated: SystemTime::now(),
            html_url: String::new(),
            description: None,
            language: None,
            stars: 0,
        }
    }
    
    /// Get the full repository name (owner/name)
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
    
    /// Get a summary of the repository's current state
    pub fn status_summary(&self) -> String {
        let workflow_status = self.latest_workflow
            .as_ref()
            .map(|w| w.status.description())
            .unwrap_or("No workflows");
        
        let pr_count = self.open_pull_requests.len();
        
        format!("{} | {} | {} open PRs", 
                self.status.description(), 
                workflow_status, 
                pr_count)
    }
    
    /// Check if the repository needs attention
    pub fn needs_attention(&self) -> bool {
        matches!(self.status, RepositoryStatus::Critical | RepositoryStatus::Warning)
    }
}

/// Configuration for repositories to monitor
#[derive(Debug, Clone)]
pub struct RepositoryConfig {
    /// Repository name
    pub name: String,
    /// Repository owner/organization
    pub owner: String,
    /// Whether to monitor this repository
    pub enabled: bool,
    /// Custom display name (optional)
    pub display_name: Option<String>,
}

impl RepositoryConfig {
    /// Create a new repository configuration
    pub fn new(name: String, owner: String) -> Self {
        Self {
            name,
            owner,
            enabled: true,
            display_name: None,
        }
    }
    
    /// Get the display name (uses custom name if set, otherwise repository name)
    pub fn display_name(&self) -> &str {
        self.display_name.as_ref().unwrap_or(&self.name)
    }
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// List of repositories to monitor
    pub repositories: Vec<RepositoryConfig>,
    /// GitHub personal access token
    pub github_token: Option<String>,
    /// Auto-refresh interval in seconds
    pub refresh_interval: u64,
    /// Maximum number of repositories to display
    pub max_repositories: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            repositories: Vec::new(),
            github_token: None,
            refresh_interval: 300, // 5 minutes
            max_repositories: 50,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_status() {
        assert_eq!(RepositoryStatus::Healthy.emoji(), "✅");
        assert_eq!(RepositoryStatus::Critical.emoji(), "❌");
        assert_eq!(RepositoryStatus::Warning.emoji(), "⚠️");
    }

    #[test]
    fn test_workflow_status() {
        assert_eq!(WorkflowStatus::Success.description(), "Passed");
        assert_eq!(WorkflowStatus::Failed.description(), "Failed");
        assert_eq!(WorkflowStatus::InProgress.description(), "Running");
    }

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new("test-repo".to_string(), "test-org".to_string());
        assert_eq!(repo.full_name(), "test-org/test-repo");
        assert_eq!(repo.status, RepositoryStatus::Unknown);
        assert!(repo.open_pull_requests.is_empty());
    }

    #[test]
    fn test_repository_config() {
        let config = RepositoryConfig::new("test".to_string(), "org".to_string());
        assert!(config.enabled);
        assert_eq!(config.display_name(), "test");
        
        let mut config_with_display = config.clone();
        config_with_display.display_name = Some("Custom Name".to_string());
        assert_eq!(config_with_display.display_name(), "Custom Name");
    }
}
