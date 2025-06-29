use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{io, time::Duration};

/// Event handling utilities
/// 
/// This module encapsulates all event polling and processing logic,
/// providing a clean interface for the main application loop.
pub struct EventHandler {
    /// Timeout for event polling in milliseconds
    poll_timeout: Duration,
}

impl EventHandler {
    /// Create a new event handler with default settings
    pub fn new() -> Self {
        Self {
            poll_timeout: Duration::from_millis(100),
        }
    }
    
    /// Create a new event handler with custom poll timeout
    pub fn with_timeout(timeout_ms: u64) -> Self {
        Self {
            poll_timeout: Duration::from_millis(timeout_ms),
        }
    }
    
    /// Poll for the next event
    /// 
    /// This function checks if any events are available within the configured timeout.
    /// It returns None if no events are available, which allows the main loop to
    /// continue without blocking indefinitely.
    /// 
    /// # Errors
    /// Returns an error if event polling fails
    pub fn next_event(&self) -> io::Result<Option<AppEvent>> {
        // Poll for events with the configured timeout
        // This prevents the app from consuming too much CPU while idle
        if event::poll(self.poll_timeout)? {
            match event::read()? {
                Event::Key(key_event) => {
                    // Only process key press events, ignore key release events
                    // This prevents double-triggering on systems that send both
                    if key_event.kind == KeyEventKind::Press {
                        Ok(Some(AppEvent::Key(key_event)))
                    } else {
                        Ok(None)
                    }
                }
                Event::Mouse(mouse_event) => {
                    // Future: Handle mouse events for enhanced interaction
                    Ok(Some(AppEvent::Mouse(mouse_event)))
                }
                Event::Resize(width, height) => {
                    // Handle terminal resize events
                    Ok(Some(AppEvent::Resize(width, height)))
                }
                _ => {
                    // Ignore other event types for now
                    Ok(None)
                }
            }
        } else {
            // No events available within timeout
            Ok(None)
        }
    }
    
    /// Get the current poll timeout
    pub fn poll_timeout(&self) -> Duration {
        self.poll_timeout
    }
    
    /// Set a new poll timeout
    pub fn set_poll_timeout(&mut self, timeout: Duration) {
        self.poll_timeout = timeout;
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Application-specific event types
/// 
/// This enum wraps the raw crossterm events in our own event type,
/// allowing us to add application-specific event handling and
/// potentially filter or transform events as needed.
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// Keyboard input event
    Key(KeyEvent),
    
    /// Mouse input event (future use)
    Mouse(ratatui::crossterm::event::MouseEvent),
    
    /// Terminal resize event
    Resize(u16, u16),
    
    // Future: Application-specific events
    // These could include things like:
    // - Timer events for auto-refresh
    // - Data update events from background tasks
    // - Network status changes
    // Refresh,
    // DataUpdated(RepositoryData),
    // NetworkStatus(bool),
}

impl AppEvent {
    /// Extract key code from a key event
    /// 
    /// Returns None if this is not a key event
    pub fn key_code(&self) -> Option<KeyCode> {
        match self {
            AppEvent::Key(key_event) => Some(key_event.code),
            _ => None,
        }
    }
    
    /// Check if this is a specific key press
    pub fn is_key(&self, target_key: KeyCode) -> bool {
        self.key_code() == Some(target_key)
    }
    
    /// Check if this is a quit event (q, Q, or Escape)
    pub fn is_quit(&self) -> bool {
        match self.key_code() {
            Some(KeyCode::Char('q')) | Some(KeyCode::Char('Q')) | Some(KeyCode::Esc) => true,
            _ => false,
        }
    }
    
    /// Check if this is a refresh event (r, R, or F5)
    pub fn is_refresh(&self) -> bool {
        match self.key_code() {
            Some(KeyCode::Char('r')) | Some(KeyCode::Char('R')) | Some(KeyCode::F(5)) => true,
            _ => false,
        }
    }
}

/// Event processing utilities
pub struct EventProcessor;

impl EventProcessor {
    /// Process a raw crossterm event and convert it to an AppEvent
    /// 
    /// This function can be used to transform or filter events before
    /// they reach the main application logic.
    pub fn process_event(event: Event) -> Option<AppEvent> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Some(AppEvent::Key(key_event))
            }
            Event::Mouse(mouse_event) => {
                Some(AppEvent::Mouse(mouse_event))
            }
            Event::Resize(width, height) => {
                Some(AppEvent::Resize(width, height))
            }
            _ => None,
        }
    }
    
    /// Check if an event should be handled immediately (like quit events)
    pub fn is_priority_event(event: &AppEvent) -> bool {
        event.is_quit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::crossterm::event::{KeyEvent, KeyModifiers};

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new();
        assert_eq!(handler.poll_timeout(), Duration::from_millis(100));
        
        let custom_handler = EventHandler::with_timeout(50);
        assert_eq!(custom_handler.poll_timeout(), Duration::from_millis(50));
    }

    #[test]
    fn test_app_event_key_detection() {
        let quit_event = AppEvent::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(quit_event.is_quit());
        assert!(!quit_event.is_refresh());
        
        let refresh_event = AppEvent::Key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
        assert!(refresh_event.is_refresh());
        assert!(!refresh_event.is_quit());
    }

    #[test]
    fn test_event_key_code_extraction() {
        let key_event = AppEvent::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(key_event.key_code(), Some(KeyCode::Enter));
        
        let resize_event = AppEvent::Resize(80, 24);
        assert_eq!(resize_event.key_code(), None);
    }
}
