# 🛡️ GitHub Repository Health Dashboard

A terminal-based dashboard built in Rust to monitor the health of GitHub repositories owned by your team. Get immediate insights into CI/CD pipeline status, pull requests, and repository activity — all from the comfort of your terminal.

## 🧭 Overview

This tool serves as both a practical utility for monitoring repository health and a learning project to gain experience with Rust, async programming, and TUI (Terminal User Interface) development using a clean, modular architecture.

## ✨ Features

### Current (MVP)
- 🖥️ **Beautiful terminal interface** using `ratatui` with responsive layout
- ⌨️ **Keyboard navigation** with intuitive controls
- 🏗️ **Modular architecture** with separation of concerns
- 🔄 **Refresh functionality** with GitHub API integration (press `r`)
- 🚪 **Graceful exit** (press `q` or `Esc`)
- 📊 **Structured data models** for repositories, workflows, and PRs
- 🧪 **Comprehensive test coverage** for all modules
- 🔗 **GitHub API integration** using `octocrab` for real-time data
- 📈 **Repository monitoring** with pull request counts and activity tracking
- 🎨 **Color-coded status indicators** (Active/Quiet/Stale)
- ⚡ **Async architecture** for non-blocking API calls

### Planned Features
- ✅ CI/CD pipeline status (success, failure, in progress)  
- 📝 Pull request tracking and review status
- 🔔 Build failure alerts and notifications
- ⚙️ Configuration file support (TOML/JSON)
- 🔄 Auto-refresh capabilities
- 📊 Historical data and trends
- 🎯 Repository filtering and selection

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.70+ recommended) - [Install Rust](https://rustup.rs/)
- **Git** - For cloning the repository
- **GitHub Personal Access Token** - For API access (when implemented)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/gh-repo-healthchecks.git
   cd gh-repo-healthchecks
   ```

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Set up GitHub token:**
   ```bash
   # Create a GitHub Personal Access Token at:
   # https://github.com/settings/tokens
   export GH_REPO_HEALTHCHECKS_TOKEN="your_github_token_here"
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```

### Development

```bash
# Run tests
cargo test

# Run with debug output
cargo run --debug

# Check code (fast compilation check)
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Generate documentation
cargo doc --open

# Build optimized release version
cargo build --release
```

## 🎮 Usage

### Controls
- **`q`** or **`Esc`** - Quit the application
- **`r`** or **`F5`** - Refresh data (ready for GitHub API integration)
- **Mouse/Touch** - Responsive to terminal resizing

### Current Interface
The application displays a beautiful three-panel layout:
- **Header**: Application title and refresh status
- **Content**: Repository table with real GitHub data showing:
  - Repository names
  - Open pull request counts (color-coded)
  - Last activity dates
  - Language and star information
  - Status indicators (Active/Quiet/Stale)
- **Footer**: Available keyboard shortcuts

### Status Indicators

- **🟢 Active**: Repository has open pull requests
- **🟡 Quiet**: Repository has recent activity but no open PRs  
- **🔴 Stale**: Repository has no recent activity

## 🏗️ Architecture

### Modular Design

Our codebase follows clean architecture principles with clear separation of concerns:

```
src/
├── main.rs              # ✅ Application entry point and coordination
├── app.rs               # ✅ Application state management
├── ui.rs                # ✅ User interface rendering and layout  
├── events.rs            # ✅ Event handling and input processing
├── terminal.rs          # ✅ Terminal setup and lifecycle management
├── models.rs            # ✅ Data structures and business logic
├── github.rs            # ✅ GitHub API integration using octocrab
└── config.rs            # 🔄 Configuration management (planned)
```

### Module Responsibilities

#### `main.rs` - Entry Point
- Application lifecycle management
- Module coordination
- Error handling and cleanup

#### `app.rs` - Application State
- Centralized state management
- Event routing and handling
- Application views and navigation

#### `ui.rs` - User Interface
- TUI rendering using `ratatui`
- Layout management and responsiveness
- Visual components and styling

#### `events.rs` - Event Processing
- Keyboard input handling
- Terminal events (resize, mouse)
- Event abstraction and routing

#### `terminal.rs` - Terminal Management
- Raw mode setup and cleanup
- Alternate screen management
- RAII-based resource management

#### `models.rs` - Data Structures
- Repository and workflow models
- Status enums and state management
- Configuration structures

#### `github.rs` - GitHub Integration  
- GitHub API client using octocrab
- Repository data fetching
- Pull request and activity monitoring
- Authentication and error handling

### Design Principles

1. **Single Responsibility**: Each module has a clear, focused purpose
2. **Dependency Injection**: Clean interfaces between modules
3. **Error Handling**: Comprehensive `Result` types throughout
4. **Resource Safety**: RAII patterns for terminal management
5. **Testability**: Unit tests for all business logic
6. **Documentation**: Extensive documentation for all public APIs

## � Current Interface Preview

```
╔═══════════════════════════════════════════════════════════════════════════╗
║     🛡️ Team Repo Health Dashboard — Press 'r' to refresh                   ║
╠═══════════════════════════════════════════════════════════════════════════╣
║ Dashboard                                                                 ║
║                                                                           ║
║                    📊 Repository Health Dashboard                        ║
║                                                                           ║
║                      🔄 Press 'r' to refresh data                        ║
║                    ❌ No repositories configured yet                      ║
║                                                                           ║
║                           Future features:                               ║
║                         • GitHub API integration                         ║
║                         • CI/CD status monitoring                        ║
║                         • Pull request tracking                          ║
║                         • Real-time updates                              ║
║                                                                           ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                  [r] Refresh  [q] Quit  [↑↓] Navigate (future)          ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

## 🔮 Planned Implementation

### Phase 1: Enhanced UI ✅ (Completed)
- [x] Modular architecture
- [x] Beautiful terminal interface  
- [x] Event handling system
- [x] Comprehensive data models
- [x] Test coverage

### Phase 2: Data Integration ✅ (Completed)
- [x] GitHub API client with authentication
- [x] Real repository data display
- [x] Pull request monitoring
- [x] Activity tracking
- [x] Error handling and retry logic

### Phase 3: Advanced Features
- [ ] Real-time status updates
- [ ] Pull request tracking
- [ ] Notification system
- [ ] Data export capabilities
- [ ] Performance monitoring

## ⚙️ Configuration (Planned)

### Environment Variables
```env
# Set your GitHub token (required)
export GH_REPO_HEALTHCHECKS_TOKEN="ghp_your_personal_access_token_here"
```

### GitHub Token Setup
1. Go to https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Select scopes:
   - `repo` (for private repositories)
   - `public_repo` (for public repositories only)
4. Copy the generated token
5. Set the environment variable before running the app

### Repository Configuration
```toml
# repos.toml
[[repositories]]
name = "auth-api"
owner = "your-org"
enabled = true

[[repositories]]
name = "billing-service"
owner = "your-org"
enabled = true
display_name = "Billing API"
```

## 🛠️ Dependencies

### Core Libraries
- **`ratatui`** - Terminal UI framework with excellent layout system
- **`crossterm`** - Cross-platform terminal manipulation
- **`octocrab`** - GitHub API client for Rust
- **`tokio`** - Async runtime for concurrent operations
- **`serde`** - Serialization for API responses
- **`chrono`** - Date and time handling

### Development Tools
- **`cargo`** - Build system and package manager
- **Built-in testing** - No external test framework dependencies
- **`rustfmt`** - Code formatting
- **`clippy`** - Linting and best practices

## 🧪 Testing

We maintain comprehensive test coverage across all modules:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific module tests
cargo test app::tests
cargo test models::tests
```

### Test Coverage
- ✅ Application state management
- ✅ Event handling logic
- ✅ Data model validation
- ✅ UI component rendering (smoke tests)
- ✅ Terminal management utilities

## 🚦 Development Workflow

### Getting Started
1. **Fork and clone** the repository
2. **Create a feature branch**: `git checkout -b feature/awesome-feature`
3. **Make your changes** following our architecture patterns
4. **Run tests**: `cargo test`
5. **Format code**: `cargo fmt`
6. **Run linter**: `cargo clippy`
7. **Commit and push**: `git commit -m 'Add awesome feature'`
8. **Create Pull Request**

### Code Style
- Use `cargo fmt` for consistent formatting
- Follow Rust naming conventions
- Add comprehensive documentation for public APIs
- Write tests for all business logic
- Handle errors explicitly with `Result` types

## 🤝 Contributing

We welcome contributions! Please read our contributing guidelines:

1. **Issues**: Report bugs or suggest features
2. **Code**: Follow our modular architecture patterns
3. **Tests**: Maintain test coverage for new features
4. **Documentation**: Update docs for API changes

### Architecture Guidelines
- Keep modules focused and single-purpose
- Use dependency injection between modules
- Handle errors gracefully with proper error types
- Write comprehensive documentation
- Follow RAII patterns for resource management

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **[ratatui](https://github.com/ratatui-org/ratatui)** - Excellent TUI framework
- **[crossterm](https://github.com/crossterm-rs/crossterm)** - Cross-platform terminal handling
- **Rust Community** - Amazing documentation and learning resources
- **GitHub API** - Enabling repository monitoring capabilities

---

**Note**: This project is actively under development with a focus on clean, modular architecture. Features marked as "planned" are not yet implemented but follow our established patterns for easy integration.
