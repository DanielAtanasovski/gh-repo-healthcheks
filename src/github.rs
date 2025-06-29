use crate::app::BackgroundMessage;
use crate::models::{
    PullRequest as AppPullRequest, PullRequestState, Repository as AppRepository, RepositoryStatus,
};
use octocrab::models::Repository;
use octocrab::Octocrab;
use std::time::SystemTime;
use tokio::sync::mpsc;

/// GitHub API client for fetching repository health data
#[derive(Debug, Clone)]
pub struct GitHubClient {
    octocrab: Octocrab,
}

impl GitHubClient {
    /// Create a new GitHub client using a personal access token
    ///
    /// The token should be set in the `GH_REPO_HEALTHCHECKS_TOKEN` environment variable
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let token = std::env::var("GH_REPO_HEALTHCHECKS_TOKEN")
            .map_err(|_| "GH_REPO_HEALTHCHECKS_TOKEN environment variable not set")?;

        let octocrab = Octocrab::builder().personal_token(token).build()?;

        Ok(Self { octocrab })
    }

    /// List all repositories for the authenticated user
    ///
    /// This fetches repositories owned by the authenticated user with additional
    /// data about pull requests, latest commits, and releases.
    pub async fn list_user_repositories(&self) -> Result<Vec<AppRepository>, String> {
        let mut repositories = Vec::new();

        // Get repositories for the authenticated user
        let repos_page = self
            .octocrab
            .current()
            .list_repos_for_authenticated_user()
            .type_("owner") // Only repositories owned by the user
            .sort("updated") // Sort by last updated
            .per_page(100) // Maximum per page
            .send()
            .await
            .map_err(|e| format!("GitHub API error: {}", e))?;

        for repo in repos_page.items {
            match self.convert_repository_with_data(repo).await {
                Ok(app_repo) => repositories.push(app_repo),
                Err(e) => eprintln!("Error processing repository: {}", e),
            }
        }

        Ok(repositories)
    }

    /// List basic repository information quickly without additional API calls
    ///
    /// This is a faster version that only fetches the basic repository list
    /// without making additional API calls for PRs, commits, etc.
    pub async fn list_basic_repositories(&self) -> Result<Vec<AppRepository>, String> {
        let mut repositories = Vec::new();

        // Get repositories for the authenticated user
        let repos_page = self
            .octocrab
            .current()
            .list_repos_for_authenticated_user()
            .type_("owner") // Only repositories owned by the user
            .sort("updated") // Sort by last updated
            .per_page(100) // Maximum per page
            .send()
            .await
            .map_err(|e| format!("GitHub API error: {}", e))?;

        for repo in repos_page.items {
            let owner = repo
                .owner
                .as_ref()
                .ok_or("Repository missing owner".to_string())?
                .login
                .clone();

            let mut app_repo = AppRepository::new(repo.name.clone(), owner.clone());

            // Set basic repository information
            app_repo.html_url = repo.html_url.map(|url| url.to_string()).unwrap_or_default();
            app_repo.description = repo.description;
            app_repo.language = repo
                .language
                .and_then(|lang| lang.as_str().map(|s| s.to_string()));
            app_repo.stars = repo.stargazers_count.unwrap_or(0) as u32;
            app_repo.last_updated = SystemTime::now();

            // Set default status
            app_repo.status = RepositoryStatus::Unknown;

            repositories.push(app_repo);
        }

        Ok(repositories)
    }

    /// Convert a GitHub repository to our app repository with additional data
    async fn convert_repository_with_data(
        &self,
        repo: Repository,
    ) -> Result<AppRepository, String> {
        let owner = repo
            .owner
            .as_ref()
            .ok_or("Repository missing owner".to_string())?
            .login
            .clone();

        let mut app_repo = AppRepository::new(repo.name.clone(), owner.clone());

        // Set basic repository information
        app_repo.html_url = repo.html_url.map(|url| url.to_string()).unwrap_or_default();
        app_repo.description = repo.description;
        app_repo.language = repo
            .language
            .and_then(|lang| lang.as_str().map(|s| s.to_string()));
        app_repo.stars = repo.stargazers_count.unwrap_or(0) as u32;
        app_repo.last_updated = SystemTime::now();

        // Fetch additional data
        match self.fetch_open_pull_requests(&owner, &repo.name).await {
            Ok(open_prs) => app_repo.open_pull_requests = open_prs,
            Err(e) => eprintln!("Failed to fetch PRs for {}/{}: {}", owner, repo.name, e),
        }

        // Fetch latest commit data
        match self.fetch_latest_commit(&owner, &repo.name).await {
            Ok(Some(commit_time)) => app_repo.latest_commit_at = Some(commit_time),
            Ok(None) => {} // No commits found
            Err(e) => eprintln!(
                "Failed to fetch latest commit for {}/{}: {}",
                owner, repo.name, e
            ),
        }

        // Determine overall repository status based on available data
        app_repo.status = self.determine_repository_status(&app_repo);

        Ok(app_repo)
    }

    /// Fetch open pull requests for a repository
    async fn fetch_open_pull_requests(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<AppPullRequest>, Box<dyn std::error::Error>> {
        let pulls_page = self
            .octocrab
            .pulls(owner, repo)
            .list()
            .state(octocrab::params::State::Open)
            .per_page(50) // Limit to first 50 open PRs
            .send()
            .await?;

        let mut app_pulls = Vec::new();

        for pr in pulls_page.items {
            let app_pr = AppPullRequest {
                number: pr.number as u32,
                title: pr.title.unwrap_or_default(),
                state: match pr.state {
                    Some(octocrab::models::IssueState::Open) => PullRequestState::Open,
                    Some(octocrab::models::IssueState::Closed) => {
                        if pr.merged_at.is_some() {
                            PullRequestState::Merged
                        } else {
                            PullRequestState::Closed
                        }
                    }
                    _ => PullRequestState::Open,
                },
                created_at: pr
                    .created_at
                    .map(|dt| {
                        SystemTime::UNIX_EPOCH
                            + std::time::Duration::from_secs(dt.timestamp() as u64)
                    })
                    .unwrap_or_else(SystemTime::now),
                updated_at: pr
                    .updated_at
                    .map(|dt| {
                        SystemTime::UNIX_EPOCH
                            + std::time::Duration::from_secs(dt.timestamp() as u64)
                    })
                    .unwrap_or_else(SystemTime::now),
                author: pr
                    .user
                    .map(|user| user.login)
                    .unwrap_or_else(|| "unknown".to_string()),
                html_url: pr.html_url.map(|url| url.to_string()).unwrap_or_default(),
                draft: pr.draft.unwrap_or(false),
                approvals: 0,         // TODO: Fetch review data
                changes_requested: 0, // TODO: Fetch review data
            };
            app_pulls.push(app_pr);
        }

        Ok(app_pulls)
    }

    /// Fetch the latest commit for a repository
    async fn fetch_latest_commit(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Option<SystemTime>, Box<dyn std::error::Error>> {
        match self
            .octocrab
            .repos(owner, repo)
            .list_commits()
            .per_page(1)
            .send()
            .await
        {
            Ok(commits_page) => {
                if let Some(commit) = commits_page.items.first() {
                    let commit_info = &commit.commit;
                    if let Some(author) = &commit_info.author {
                        if let Some(date) = author.date {
                            let timestamp = date.timestamp() as u64;
                            return Ok(Some(
                                SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(timestamp),
                            ));
                        }
                    }
                }
                Ok(None)
            }
            Err(_) => Ok(None), // If we can't fetch commits, just return None
        }
    }

    /// Fetch the latest release for a repository  
    /// This is a placeholder for future implementation
    async fn fetch_latest_release(
        &self,
        _owner: &str,
        _repo: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        // TODO: Implement fetching latest release data
        // let releases = self.octocrab.repos(owner, repo).releases().list().per_page(1).send().await?;
        Ok(None)
    }

    /// Fetch recent workflow runs for a repository
    async fn fetch_workflow_runs(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<crate::models::WorkflowRun>, Box<dyn std::error::Error>> {
        // For now, return empty vector as octocrab doesn't have direct workflow support
        // This would need to be implemented with raw HTTP requests to GitHub Actions API
        // or wait for octocrab to add workflow support
        
        // Placeholder implementation - in a real scenario, you'd call:
        // GET /repos/{owner}/{repo}/actions/runs
        
        Ok(Vec::new())
    }

    /// Determine the overall status of a repository based on last commit time
    fn determine_repository_status(&self, repo: &AppRepository) -> RepositoryStatus {
        RepositoryStatus::from_last_commit(repo.latest_commit_at)
    }

    /// Get the authenticated user information for testing
    pub async fn get_current_user(&self) -> Result<String, Box<dyn std::error::Error>> {
        let user = self.octocrab.current().user().await?;
        Ok(user.login)
    }

    /// Spawn a background task to fetch repositories progressively
    pub fn spawn_background_fetch(
        client: GitHubClient,
        sender: mpsc::UnboundedSender<BackgroundMessage>,
    ) {
        tokio::spawn(async move {
            // Phase 1: Fetch basic repository information quickly
            let basic_result = client.list_basic_repositories().await;
            
            match basic_result {
                Ok(basic_repositories) => {
                    let total = basic_repositories.len();
                    
                    // Send start message
                    if sender
                        .send(BackgroundMessage::FetchStarted { total })
                        .is_err()
                    {
                        return; // Receiver dropped
                    }
                    
                    // Send each basic repository immediately
                    for (i, repository) in basic_repositories.iter().enumerate() {
                        if sender
                            .send(BackgroundMessage::RepositoryFetched {
                                repository: repository.clone(),
                                current: i + 1,
                                total,
                            })
                            .is_err()
                        {
                            return; // Receiver dropped
                        }
                    }
                    
                    // Send initial completion to show the basic list
                    if sender
                        .send(BackgroundMessage::FetchCompleted { 
                            repositories: basic_repositories.clone() 
                        })
                        .is_err()
                    {
                        return; // Receiver dropped
                    }
                    
                    // Phase 2: Enhance repositories with additional data
                    let mut enhanced_repositories = basic_repositories.clone();
                    
                    // Update the UI to show we're enhancing repositories
                    if sender
                        .send(BackgroundMessage::EnhancementStarted { 
                            total 
                        })
                        .is_err()
                    {
                        return; // Receiver dropped
                    }
                    
                    // Enhance each repository with additional details
                    for (i, repo) in enhanced_repositories.iter_mut().enumerate() {
                        // Enhance this repository with additional data
                        if let Err(e) = client.enhance_repository(repo).await {
                            eprintln!("Error enhancing repository {}: {}", repo.name, e);
                        }
                        
                        // Send update for this enhanced repository
                        if sender
                            .send(BackgroundMessage::RepositoryEnhanced {
                                repository: repo.clone(),
                                current: i + 1,
                                total,
                            })
                            .is_err()
                        {
                            return; // Receiver dropped
                        }
                        
                        // Small delay to allow UI updates and prevent API rate limiting
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                    
                    // Send final completion message with all enhanced data
                    let _ = sender.send(BackgroundMessage::EnhancementCompleted { 
                        repositories: enhanced_repositories 
                    });
                }
                Err(e) => {
                    let error_msg = format!("Failed to fetch repositories: {}", e);
                    let _ = sender.send(BackgroundMessage::FetchError { error: error_msg });
                }
            }
        });
    }

    /// Enhance a repository with additional data like PRs and commits
    pub async fn enhance_repository(&self, repo: &mut AppRepository) -> Result<(), String> {
        // Fetch additional data
        match self.fetch_open_pull_requests(&repo.owner, &repo.name).await {
            Ok(open_prs) => repo.open_pull_requests = open_prs,
            Err(e) => eprintln!("Failed to fetch PRs for {}/{}: {}", repo.owner, repo.name, e),
        }

        // Fetch latest commit data
        match self.fetch_latest_commit(&repo.owner, &repo.name).await {
            Ok(Some(commit_time)) => repo.latest_commit_at = Some(commit_time),
            Ok(None) => {} // No commits found
            Err(e) => eprintln!(
                "Failed to fetch latest commit for {}/{}: {}",
                repo.owner, repo.name, e
            ),
        }

        // Fetch workflow runs
        match self.fetch_workflow_runs(&repo.owner, &repo.name).await {
            Ok(workflows) => {
                repo.recent_workflows = workflows.clone();
                repo.workflow_health = crate::models::WorkflowHealth::from_workflow_runs(&workflows);
                repo.latest_workflow = workflows.first().cloned();
            }
            Err(e) => eprintln!(
                "Failed to fetch workflows for {}/{}: {}",
                repo.owner, repo.name, e
            ),
        }

        // Determine overall repository status based on commit activity
        repo.status = self.determine_repository_status(&repo);

        Ok(())
    }
}

/// Error type for GitHub API operations
#[derive(Debug)]
pub enum GitHubError {
    /// Authentication failed
    AuthenticationFailed,
    /// API rate limit exceeded
    RateLimitExceeded,
    /// Network error
    NetworkError(String),
    /// Repository not found
    RepositoryNotFound(String),
    /// General API error
    ApiError(String),
}

impl std::fmt::Display for GitHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitHubError::AuthenticationFailed => write!(f, "GitHub authentication failed"),
            GitHubError::RateLimitExceeded => write!(f, "GitHub API rate limit exceeded"),
            GitHubError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            GitHubError::RepositoryNotFound(repo) => write!(f, "Repository not found: {}", repo),
            GitHubError::ApiError(msg) => write!(f, "GitHub API error: {}", msg),
        }
    }
}

impl std::error::Error for GitHubError {}

impl From<octocrab::Error> for GitHubError {
    fn from(error: octocrab::Error) -> Self {
        match error {
            octocrab::Error::GitHub { source, .. } => {
                if source.message.contains("rate limit") {
                    GitHubError::RateLimitExceeded
                } else if source.message.contains("authentication")
                    || source.message.contains("401")
                {
                    GitHubError::AuthenticationFailed
                } else {
                    GitHubError::ApiError(source.message)
                }
            }
            octocrab::Error::Http { source, .. } => GitHubError::NetworkError(source.to_string()),
            _ => GitHubError::ApiError(error.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_github_error_display() {
        let auth_error = GitHubError::AuthenticationFailed;
        assert_eq!(auth_error.to_string(), "GitHub authentication failed");

        let rate_limit_error = GitHubError::RateLimitExceeded;
        assert_eq!(
            rate_limit_error.to_string(),
            "GitHub API rate limit exceeded"
        );
    }

    #[tokio::test]
    async fn test_github_client_creation_without_token() {
        // This test should fail if the token is not set
        // Remove the environment variable for this test
        std::env::remove_var("GH_REPO_HEALTHCHECKS_TOKEN");

        let result = GitHubClient::new();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("GH_REPO_HEALTHCHECKS_TOKEN"));
    }

    // Note: Integration tests with real GitHub API would require a valid token
    // and should be run separately from unit tests
}
