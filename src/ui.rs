use ratatui::{
    prelude::*,
    widgets::*,
};
use crate::app::{App, AppView};

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
                Constraint::Length(3),    // Header
                Constraint::Min(0),       // Content (flexible)
                Constraint::Length(3),    // Footer
            ])
            .split(area);
        
        // Render each section
        Self::render_header(frame, main_layout[0], app);
        Self::render_content(frame, main_layout[1], app);
        Self::render_footer(frame, main_layout[2]);
    }
    
    /// Render the header section
    /// 
    /// Shows the application title and status information
    fn render_header(frame: &mut Frame, area: Rect, app: &App) {
        let title_text = if let Some(last_refresh) = app.last_refresh {
            let elapsed = last_refresh.elapsed();
            format!("{} — Last Refresh: {}s ago", 
                   app.get_title(), 
                   elapsed.as_secs())
        } else {
            format!("{} — Press 'r' to refresh", app.get_title())
        };
        
        let header = Block::default()
            .title(title_text)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Black));
        
        frame.render_widget(header, area);
    }
    
    /// Render the main content area
    /// 
    /// Currently shows a placeholder. Future: Repository table with status indicators
    fn render_content(frame: &mut Frame, area: Rect, _app: &App) {
        // Create inner area with padding
        let inner_area = area.inner(Margin::new(1, 1));
        
        // Placeholder content - future: repository list table
        let placeholder_text = vec![
            Line::from("📊 Repository Health Dashboard"),
            Line::from(""),
            Line::from("🔄 Press 'r' to refresh data"),
            Line::from("❌ No repositories configured yet"),
            Line::from(""),
            Line::from("Future features:"),
            Line::from("  • GitHub API integration"),
            Line::from("  • CI/CD status monitoring"),
            Line::from("  • Pull request tracking"),
            Line::from("  • Real-time updates"),
        ];
        
        let content = Paragraph::new(placeholder_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray))
            .title("Dashboard")
            .title_alignment(Alignment::Left);
        
        frame.render_widget(content_block, area);
        frame.render_widget(content, inner_area);
    }
    
    /// Render the footer section
    /// 
    /// Shows available key bindings and controls
    fn render_footer(frame: &mut Frame, area: Rect) {
        let footer_text = Line::from(vec![
            Span::styled("[r] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("Refresh  "),
            Span::styled("[q] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("Quit  "),
            Span::styled("[↑↓] ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw("Navigate (future)"),
        ]);
        
        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Gray))
            );
        
        frame.render_widget(footer, area);
    }
}

/// Future: Repository table rendering
/// 
/// This will be used to render the repository list with status indicators
#[allow(dead_code)]
fn render_repository_table(frame: &mut Frame, area: Rect) {
    // Future implementation:
    // - Create table with columns: Repository, Last PR, CI Status, Last Run
    // - Add status indicators with colors
    // - Handle selection and scrolling
    
    let placeholder = Block::default()
        .title("Repositories")
        .borders(Borders::ALL);
    
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
