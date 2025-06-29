use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        event::{DisableMouseCapture, EnableMouseCapture},
    },
    prelude::*,
};
use std::{error::Error, io};

/// Terminal management utilities
/// 
/// This module encapsulates all terminal setup, cleanup, and management
/// functionality, keeping it separate from the main application logic.
pub struct TerminalManager;

impl TerminalManager {
    /// Initialize and configure the terminal for TUI mode
    /// 
    /// This function:
    /// - Enables raw mode (disables line buffering and echo)
    /// - Switches to alternate screen (preserves user's terminal content)
    /// - Enables mouse capture for potential future mouse support
    /// - Creates and returns a configured Terminal instance
    /// 
    /// # Errors
    /// Returns an error if terminal initialization fails
    pub fn setup() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
        // Enable raw mode - this allows us to read key presses immediately
        // without waiting for Enter, and disables echoing characters to screen
        enable_raw_mode()?;
        
        let mut stdout = io::stdout();
        
        // Enter alternate screen - this creates a new screen buffer so we don't
        // interfere with the user's existing terminal content
        // Enable mouse capture for potential future features
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        // Create the terminal backend and terminal instance
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }
    
    /// Restore terminal to its original state
    /// 
    /// This function should be called when exiting the application to ensure
    /// the terminal is left in a clean state:
    /// - Disables raw mode
    /// - Returns to the main screen (exits alternate screen)
    /// - Disables mouse capture
    /// - Shows the cursor again
    /// 
    /// # Errors
    /// Returns an error if terminal cleanup fails
    pub fn cleanup(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn Error>> {
        // Disable raw mode to restore normal terminal behavior
        disable_raw_mode()?;
        
        // Exit alternate screen and disable mouse capture
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        
        // Make sure the cursor is visible when we exit
        terminal.show_cursor()?;

        Ok(())
    }
    
    /// Setup terminal with automatic cleanup on drop
    /// 
    /// Returns a TerminalGuard that will automatically cleanup the terminal
    /// when dropped, ensuring cleanup even if the application panics.
    pub fn setup_with_guard() -> Result<TerminalGuard, Box<dyn Error>> {
        let terminal = Self::setup()?;
        Ok(TerminalGuard::new(terminal))
    }
}

/// RAII guard for terminal cleanup
/// 
/// This struct ensures that terminal cleanup happens automatically
/// when the guard goes out of scope, even if the application panics.
pub struct TerminalGuard {
    terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
}

impl TerminalGuard {
    /// Create a new terminal guard
    fn new(terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Self {
        Self {
            terminal: Some(terminal),
        }
    }
    
    /// Get a mutable reference to the terminal
    /// 
    /// # Panics
    /// Panics if the terminal has already been taken (which should never happen
    /// in normal usage)
    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        self.terminal.as_mut().expect("Terminal should be available")
    }
    
    /// Manually cleanup the terminal
    /// 
    /// This allows for explicit cleanup and error handling. If not called,
    /// cleanup will happen automatically when the guard is dropped.
    pub fn cleanup(mut self) -> Result<(), Box<dyn Error>> {
        if let Some(mut terminal) = self.terminal.take() {
            TerminalManager::cleanup(&mut terminal)?;
        }
        Ok(())
    }
}

impl Drop for TerminalGuard {
    /// Automatic cleanup when the guard goes out of scope
    /// 
    /// This ensures cleanup happens even if the application panics.
    /// Errors during cleanup are logged but not propagated since
    /// we're in a Drop implementation.
    fn drop(&mut self) {
        if let Some(mut terminal) = self.terminal.take() {
            if let Err(err) = TerminalManager::cleanup(&mut terminal) {
                eprintln!("Error during terminal cleanup: {:?}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_manager_exists() {
        // Basic test to ensure the module compiles
        // We can't easily test actual terminal operations in a unit test
        // without mocking the terminal
        let _manager = TerminalManager;
    }
    
    // Note: Integration tests for terminal functionality would be better
    // placed in a separate integration test file where we can control
    // the test environment more carefully
}
