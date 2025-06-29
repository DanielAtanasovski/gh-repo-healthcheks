# 🛡️ GitHub Repository Health Dashboard

A terminal-based dashboard built in Rust to monitor the health of GitHub repositories owned by your team. Get immediate insights into CI/CD pipeline status, pull requests, and repository activity — all from the comfort of your terminal.

## 🧭 Overview

This tool serves as both a practical utility for monitoring repository health and a learning project to gain experience with Rust, async programming, and TUI (Terminal User Interface) development using a clean, modular architecture.

## ✨ Features

### Current (MVP)
- 🖥️ **Beautiful terminal interface** using `ratatui` with responsive layout
- ⌨️ **Keyboard navigation** with intuitive controls
- 🏗️ **Modular architecture** with separation of concerns
- 🔄 **Refresh functionality** (press `r` - infrastructure ready)
- 🚪 **Graceful exit** (press `q` or `Esc`)
- 📊 **Structured data models** for repositories, workflows, and PRs
- 🧪 **Comprehensive test coverage** for all modules

### Planned Features
- 🔗 GitHub API integration for real-time data
- ✅ CI/CD pipeline status (success, failure, in progress)  
- 📝 Pull request tracking and review status
- 🔔 Build failure alerts and notifications
- ⚙️ Configuration file support (TOML/JSON)
- 🔄 Auto-refresh capabilities
- 📊 Historical data and trends

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

3. **Run the application:**
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
- **Content**: Repository dashboard (currently showing placeholder content)
- **Footer**: Available keyboard shortcuts

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
├── github.rs            # 🔄 GitHub API integration (planned)
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

### Phase 1: Enhanced UI ✅ (Current)
- [x] Modular architecture
- [x] Beautiful terminal interface  
- [x] Event handling system
- [x] Comprehensive data models
- [x] Test coverage

### Phase 2: Data Integration (In Progress)
- [ ] GitHub API client with authentication
- [ ] Configuration file loading (repos.toml)
- [ ] Real repository data display
- [ ] Error handling and retry logic

### Phase 3: Advanced Features
- [ ] Real-time status updates
- [ ] Pull request tracking
- [ ] Notification system
- [ ] Data export capabilities
- [ ] Performance monitoring

## ⚙️ Configuration (Planned)

### Environment Variables
```env
# .env file
GITHUB_TOKEN=ghp_your_personal_access_token_here
```

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
- **Future additions**:
  - `reqwest` - HTTP client for GitHub API
  - `tokio` - Async runtime for concurrent operations
  - `serde` - Serialization for configuration and API responses
  - `toml` - Configuration file parsing

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
