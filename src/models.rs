use std::time::{Duration, SystemTime};

/// Represents the activity level of a repository based on last commit time
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryStatus {
    /// Committed today (HOT)
    Hot,
    /// Committed within the last week
    Active,
    /// Committed within the last month
    Moderate,
    /// Committed within the last 3 months
    Quiet,
    /// Committed within the last 6 months
    Stale,
    /// No commits in over 6 months
    Dormant,
    /// Status is unknown or being fetched
    Unknown,
}

impl RepositoryStatus {
    /// Get a human-readable description of the status
    pub fn description(&self) -> &'static str {
        match self {
            RepositoryStatus::Hot => "Very active (today)",
            RepositoryStatus::Active => "Active (this week)",
            RepositoryStatus::Moderate => "Moderate activity (this month)",
            RepositoryStatus::Quiet => "Quiet (last 3 months)",
            RepositoryStatus::Stale => "Stale (last 6 months)",
            RepositoryStatus::Dormant => "Dormant (6+ months)",
            RepositoryStatus::Unknown => "Status unknown",
        }
    }

    /// Get an emoji representation of the status
    pub fn emoji(&self) -> &'static str {
        match self {
            RepositoryStatus::Hot => "🔥",
            RepositoryStatus::Active => "⚡",
            RepositoryStatus::Moderate => "✅",
            RepositoryStatus::Quiet => "⚠️",
            RepositoryStatus::Stale => "🟡",
            RepositoryStatus::Dormant => "💤",
            RepositoryStatus::Unknown => "❓",
        }
    }

    /// Get color for UI rendering
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            RepositoryStatus::Hot => ratatui::style::Color::Red,
            RepositoryStatus::Active => ratatui::style::Color::Green,
            RepositoryStatus::Moderate => ratatui::style::Color::Cyan,
            RepositoryStatus::Quiet => ratatui::style::Color::Yellow,
            RepositoryStatus::Stale => ratatui::style::Color::Magenta,
            RepositoryStatus::Dormant => ratatui::style::Color::DarkGray,
            RepositoryStatus::Unknown => ratatui::style::Color::Gray,
        }
    }

    /// Determine status from last commit time
    pub fn from_last_commit(last_commit: Option<SystemTime>) -> Self {
        match last_commit {
            Some(commit_time) => {
                let now = SystemTime::now();
                if let Ok(duration) = now.duration_since(commit_time) {
                    let days = duration.as_secs() / (24 * 60 * 60);
                    
                    match days {
                        0 => RepositoryStatus::Hot,
                        1..=7 => RepositoryStatus::Active,
                        8..=30 => RepositoryStatus::Moderate,
                        31..=90 => RepositoryStatus::Quiet,
                        91..=180 => RepositoryStatus::Stale,
                        _ => RepositoryStatus::Dormant,
                    }
                } else {
                    RepositoryStatus::Unknown
                }
            }
            None => RepositoryStatus::Unknown,
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

/// Represents the overall workflow health of a repository
#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowHealth {
    /// All workflows passing or no workflows
    Excellent,
    /// Most workflows passing (>80%)
    Good,
    /// Some workflows failing (50-80% passing)
    Fair,
    /// Many workflows failing (<50% passing)
    Poor,
    /// All workflows failing
    Critical,
    /// No workflow data available
    Unknown,
}

impl WorkflowHealth {
    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            WorkflowHealth::Excellent => "All workflows passing",
            WorkflowHealth::Good => "Most workflows passing",
            WorkflowHealth::Fair => "Some workflows failing",
            WorkflowHealth::Poor => "Many workflows failing",
            WorkflowHealth::Critical => "All workflows failing",
            WorkflowHealth::Unknown => "No workflow data",
        }
    }

    /// Get an emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            WorkflowHealth::Excellent => "✅",
            WorkflowHealth::Good => "🟢",
            WorkflowHealth::Fair => "🟡",
            WorkflowHealth::Poor => "🟠",
            WorkflowHealth::Critical => "🔴",
            WorkflowHealth::Unknown => "❓",
        }
    }

    /// Get color for UI rendering
    pub fn color(&self) -> ratatui::style::Color {
        match self {
            WorkflowHealth::Excellent => ratatui::style::Color::Green,
            WorkflowHealth::Good => ratatui::style::Color::Cyan,
            WorkflowHealth::Fair => ratatui::style::Color::Yellow,
            WorkflowHealth::Poor => ratatui::style::Color::LightRed,
            WorkflowHealth::Critical => ratatui::style::Color::Red,
            WorkflowHealth::Unknown => ratatui::style::Color::Gray,
        }
    }

    /// Calculate workflow health from a list of recent workflow runs
    pub fn from_workflow_runs(workflows: &[WorkflowRun]) -> Self {
        if workflows.is_empty() {
            return WorkflowHealth::Excellent; // No workflows = no failures
        }

        let total = workflows.len() as f32;
        let successful = workflows
            .iter()
            .filter(|w| matches!(w.status, WorkflowStatus::Success))
            .count() as f32;
        
        let success_rate = successful / total;
        
        match success_rate {
            r if r >= 1.0 => WorkflowHealth::Excellent,
            r if r >= 0.8 => WorkflowHealth::Good,
            r if r >= 0.5 => WorkflowHealth::Fair,
            r if r > 0.0 => WorkflowHealth::Poor,
            _ => WorkflowHealth::Critical,
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
    /// Overall health status based on commit activity
    pub status: RepositoryStatus,
    /// Overall workflow health based on recent runs
    pub workflow_health: WorkflowHealth,
    /// Most recent workflow run
    pub latest_workflow: Option<WorkflowRun>,
    /// Recent workflow runs for health calculation
    pub recent_workflows: Vec<WorkflowRun>,
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
    /// Latest commit timestamp
    pub latest_commit_at: Option<SystemTime>,
}

impl Repository {
    /// Create a new repository with default values
    pub fn new(name: String, owner: String) -> Self {
        Self {
            name,
            owner,
            status: RepositoryStatus::Unknown,
            workflow_health: WorkflowHealth::Unknown,
            latest_workflow: None,
            recent_workflows: Vec::new(),
            open_pull_requests: Vec::new(),
            last_updated: SystemTime::now(),
            html_url: String::new(),
            description: None,
            language: None,
            stars: 0,
            latest_commit_at: None,
        }
    }

    /// Get the full repository name (owner/name)
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }

    /// Get a summary of the repository's current state
    pub fn status_summary(&self) -> String {
        let workflow_status = self
            .latest_workflow
            .as_ref()
            .map(|w| w.status.description())
            .unwrap_or("No workflows");

        let pr_count = self.open_pull_requests.len();

        format!(
            "{} | {} | {} open PRs",
            self.status.description(),
            workflow_status,
            pr_count
        )
    }

    /// Check if the repository needs attention
    pub fn needs_attention(&self) -> bool {
        matches!(
            self.status,
            RepositoryStatus::Stale | RepositoryStatus::Dormant
        ) || !self.open_pull_requests.is_empty()
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
        assert_eq!(RepositoryStatus::Hot.emoji(), "🔥");
        assert_eq!(RepositoryStatus::Active.emoji(), "⚡");
        assert_eq!(RepositoryStatus::Dormant.emoji(), "💤");
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

    #[test]
    fn test_workflow_health() {
        let run1 = WorkflowRun {
            id: 1,
            name: "CI".to_string(),
            status: WorkflowStatus::Success,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            duration: Some(Duration::from_secs(60)),
            conclusion: Some("success".to_string()),
            html_url: "http://example.com".to_string(),
        };

        let run2 = WorkflowRun {
            id: 2,
            name: "CD".to_string(),
            status: WorkflowStatus::Failed,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            duration: Some(Duration::from_secs(120)),
            conclusion: Some("failure".to_string()),
            html_url: "http://example.com".to_string(),
        };

        assert_eq!(WorkflowHealth::from_workflow_runs(&[run1.clone()]), WorkflowHealth::Excellent);
        assert_eq!(WorkflowHealth::from_workflow_runs(&[run1, run2]), WorkflowHealth::Fair);
    }
}
