use crate::app::{App, AppView};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;

/// Main UI renderer
///
/// This module handles all UI rendering logic, keeping it separate from
/// the application state and main event loop.
pub struct UI;

impl UI {
    /// Render the main application UI
    ///
    /// This function determines which view to render based on the app state
    /// and delegates to the appropriate rendering function.
    pub fn render(frame: &mut Frame, app: &App) {
        match app.current_view {
            AppView::Dashboard => Self::render_dashboard(frame, app),
            // Future views:
            // AppView::Settings => Self::render_settings(frame, app),
            // AppView::RepoDetails => Self::render_repo_details(frame, app),
        }
    }

    /// Render the main dashboard view
    ///
    /// This creates the primary layout with:
    /// - Header with title and status
    /// - Main content area (repository list)
    /// - Footer with key bindings
    fn render_dashboard(frame: &mut Frame, app: &App) {
        let area = frame.area();

        // Create the main layout: header, content, footer
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content (flexible)
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // Render each section
        Self::render_header(frame, main_layout[0], app);
        Self::render_content(frame, main_layout[1], app);
        Self::render_footer(frame, main_layout[2], app);
    }

    /// Render the header section
    ///
    /// Shows the application title and status information
    fn render_header(frame: &mut Frame, area: Rect, app: &App) {
        let header_block = Block::default()
            .title(app.get_title())
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Black));

        // Create inner area for content
        let inner_area = header_block.inner(area);
        
        // Render the block first
        frame.render_widget(header_block, area);

        // Create status text lines
        let mut status_lines = Vec::new();
        
        // Repository count info
        if app.repositories.is_empty() && !app.is_loading() {
            status_lines.push(Line::from("No repositories found"));
        } else if app.is_loading() && app.repositories.is_empty() {
            status_lines.push(Line::from("Loading repositories..."));
        } else {
            let active_count = app
                .repositories
                .iter()
                .filter(|repo| !repo.open_pull_requests.is_empty())
                .count();

            status_lines.push(Line::from(format!(
                "{} repositories ({} with active PRs)",
                app.repositories.len(),
                active_count
            )));
        }

        // Last refresh info
        if let Some(last_refresh) = app.last_refresh {
            let elapsed = last_refresh.elapsed();
            let refresh_text = if elapsed.as_secs() < 60 {
                format!("Last refresh: {}s ago", elapsed.as_secs())
            } else {
                format!("Last refresh: {}m ago", elapsed.as_secs() / 60)
            };
            status_lines.push(Line::from(refresh_text));
        }

        // Render the status text
        let status_paragraph = Paragraph::new(status_lines)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        frame.render_widget(status_paragraph, inner_area);
    }

    /// Render the main content area
    ///
    /// Shows repository list with status indicators
    fn render_content(frame: &mut Frame, area: Rect, app: &App) {
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray))
            .title("Repositories")
            .title_alignment(Alignment::Left);

        frame.render_widget(content_block, area);

        // Create inner area with padding
        let inner_area = area.inner(Margin::new(1, 1));

        if app.is_loading() && app.repositories.is_empty() {
            // Show loading indicator with progress ONLY if we don't have any repositories yet
            let mut loading_text = vec![
                Line::from(""),
                Line::from("🔄 Loading repositories..."),
                Line::from(""),
            ];

            // Add progress information if available
            if let Some((current, total)) = app.loading_progress {
                loading_text.push(Line::from(format!(
                    "Progress: {} / {} repositories",
                    current, total
                )));

                // Create a simple progress bar
                let progress_width = 40;
                let filled = if total > 0 {
                    (current * progress_width) / total
                } else {
                    0
                };
                let empty = progress_width - filled;

                let progress_bar = format!(
                    "[{}{}] {:.1}%",
                    "█".repeat(filled),
                    "░".repeat(empty),
                    if total > 0 {
                        (current as f64 / total as f64) * 100.0
                    } else {
                        0.0
                    }
                );

                loading_text.push(Line::from(""));
                loading_text.push(Line::from(progress_bar));
            } else {
                loading_text.push(Line::from(
                    "This may take a moment while we fetch data from GitHub.",
                ));
            }

            let loading = Paragraph::new(loading_text)
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center);

            frame.render_widget(loading, inner_area);
        } else if let Some(error) = app.get_error_message() {
            // Show error message
            let error_text = vec![
                Line::from(""),
                Line::from("❌ Error loading repositories"),
                Line::from(""),
                Line::from(error),
                Line::from(""),
                Line::from("Press 'r' to retry"),
            ];

            let error_paragraph = Paragraph::new(error_text)
                .style(Style::default().fg(Color::Red))
                .alignment(Alignment::Center);

            frame.render_widget(error_paragraph, inner_area);
        } else if app.repository_count() == 0 {
            // Show empty state
            let empty_text = vec![
                Line::from(""),
                Line::from("📂 No repositories found"),
                Line::from(""),
                Line::from("Make sure your GitHub token has access to repositories."),
                Line::from(""),
                Line::from("Press 'r' to refresh"),
            ];

            let empty = Paragraph::new(empty_text)
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);

            frame.render_widget(empty, inner_area);
        } else {
            // Show repository table
            Self::render_repository_table(frame, inner_area, app);

            // If we're enhancing repositories, show an enhancement indicator in the corner
            if app.is_enhancing {
                let enhancement_indicator = match app.enhancement_progress {
                    Some((current, total)) => {
                        format!("Enhancing: {}/{} repos", current, total)
                    }
                    None => "Enhancing...".to_string(),
                };

                // Create a small floating widget for the enhancement status
                let indicator_height = 3;
                let indicator_width = enhancement_indicator.len() as u16 + 4;
                let indicator_x = area.width.saturating_sub(indicator_width);
                let indicator_y = 0;

                let indicator_area = Rect::new(
                    area.x + indicator_x,
                    area.y + indicator_y,
                    indicator_width,
                    indicator_height,
                );

                let indicator_widget = Paragraph::new(enhancement_indicator)
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::Yellow))
                            .style(Style::default().bg(Color::Black)),
                    );

                frame.render_widget(indicator_widget, indicator_area);
            }
        }
    }

    /// Render the footer section
    ///
    /// Shows available key bindings and controls
    fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
        let mut controls = vec![
            Span::styled(
                "[r] ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Refresh  "),
            Span::styled(
                "[q] ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw("Quit  "),
            Span::styled(
                "[↑↓] ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("Navigate  "),
        ];

        // Add pagination info if we have repositories
        if !app.repositories.is_empty() {
            let page_info = format!(
                "({}/{} repos) ",
                app.scroll_offset + 1,
                app.repositories.len()
            );
            controls.push(Span::styled(
                page_info,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));

            // Page up/down controls
            controls.push(Span::styled(
                "[PgUp/PgDn] ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ));
            controls.push(Span::raw("Page  "));

            // Home/End controls
            controls.push(Span::styled(
                "[Home/End] ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ));
            controls.push(Span::raw("Top/Bottom"));

            // Enhancement status
            if app.is_enhancing {
                if let Some((current, total)) = app.enhancement_progress {
                    controls.push(Span::raw("  "));
                    controls.push(Span::styled(
                        format!("Enhancing: {}/{}", current, total),
                        Style::default().fg(Color::Yellow),
                    ));
                }
            }
        }

        let footer_text = Line::from(controls);

        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray)),
            );

        frame.render_widget(footer, area);
    }
}

impl UI {
    /// Render the repository table with actual data
    fn render_repository_table(frame: &mut Frame, area: Rect, app: &App) {
        let repositories = app.get_repositories();

        if repositories.is_empty() {
            return;
        }

        // Calculate visible height and visible items
        let visible_height = area.height as usize;
        let visible_items = app.get_visible_item_count(visible_height);

        // Get scroll window based on current offset
        let start_index = app.scroll_offset;
        let end_index = (app.scroll_offset + visible_items).min(repositories.len());

        // Create table headers
        let header = Row::new(vec![
            Cell::from("Repository").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Cell::from("PRs").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Cell::from("Last Activity").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Cell::from("Info").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Cell::from("Workflows").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Cell::from("Status").style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        // Create table rows from visible repository data
        let rows: Vec<Row> = repositories
            .iter()
            .enumerate()
            .skip(start_index)
            .take(end_index - start_index)
            .map(|(index, repo)| {
                // Format pull request count
                let pr_count = if repo.open_pull_requests.is_empty() {
                    "0".to_string()
                } else {
                    repo.open_pull_requests.len().to_string()
                };

                // Format last commit date - use actual commit data now
                let last_activity = if let Some(commit_time) = repo.latest_commit_at {
                    let duration = commit_time.elapsed().unwrap_or(Duration::from_secs(0));
                    let days_ago = duration.as_secs() / 86400; // seconds in a day
                    if days_ago == 0 {
                        "Today".to_string()
                    } else if days_ago == 1 {
                        "1 day ago".to_string()
                    } else if days_ago < 7 {
                        format!("{} days ago", days_ago)
                    } else {
                        format!("{} weeks ago", days_ago / 7)
                    }
                } else {
                    "No commits".to_string()
                };

                // Format repository language and stars info
                let info = match (&repo.language, repo.stars) {
                    (Some(lang), stars) if stars > 0 => format!("{} ({} ⭐)", lang, stars),
                    (Some(lang), _) => lang.clone(),
                    (None, stars) if stars > 0 => format!("{} ⭐", stars),
                    _ => "N/A".to_string(),
                };

                // Format workflow status
                let workflow_status = format!(
                    "{} {}",
                    repo.workflow_health.emoji(),
                    repo.workflow_health.description()
                );

                // Determine status based on commit activity
                let status_text = format!("{} {}", repo.status.emoji(), repo.status.description());

                // Apply selection highlighting
                let row_style = if app.selected_repository == index {
                    Style::default().bg(Color::Blue).fg(Color::White)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(repo.name.as_str()),
                    Cell::from(pr_count).style(Style::default().fg(
                        if repo.open_pull_requests.is_empty() {
                            Color::Gray
                        } else {
                            Color::Green
                        },
                    )),
                    Cell::from(last_activity),
                    Cell::from(info),
                    Cell::from(workflow_status)
                        .style(Style::default().fg(repo.workflow_health.color())),
                    Cell::from(status_text).style(Style::default().fg(repo.status.color())),
                ])
                .style(row_style)
            })
            .collect();

        // Create the table widget
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(25), // Repository name
                Constraint::Percentage(8),  // PR count
                Constraint::Percentage(15), // Last activity
                Constraint::Percentage(17), // Info
                Constraint::Percentage(20), // Workflow status
                Constraint::Percentage(15), // Status
            ],
        )
        .header(header)
        .block(Block::default().borders(Borders::NONE))
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

        frame.render_widget(table, area);
    }
}

/// Future: Repository table rendering
///
/// This will be used to render the repository list with status indicators
#[allow(dead_code)]
fn render_repository_table_old(frame: &mut Frame, area: Rect) {
    // Future implementation:
    // - Create table with columns: Repository, Last PR, CI Status, Last Run
    // - Add status indicators with colors
    // - Handle selection and scrolling

    let placeholder = Block::default().title("Repositories").borders(Borders::ALL);

    frame.render_widget(placeholder, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;

    #[test]
    fn test_ui_render_does_not_panic() {
        // This is a basic smoke test to ensure UI rendering doesn't panic
        // More comprehensive UI tests would require a mock terminal
        let app = App::new();

        // We can't easily test the actual rendering without a terminal,
        // but we can test that our UI struct can be created
        let _ui = UI;

        // Verify app state is as expected
        assert!(!app.should_quit());
        assert_eq!(app.current_view, AppView::Dashboard);
    }
}
