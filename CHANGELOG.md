# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.6] - 2025-08-30

### Added
- `tidy` command to automatically delete files older than 30 days from trash
- Warning prompts before permanently deleting files from trash
- Enhanced recovery functionality with `recover` and `recover-all` commands
- Better trash content listing with improved formatting

### Changed
- Improved input argument handling and validation
- Enhanced function organization and code structure
- Updated documentation with comprehensive usage examples

### Fixed
- Input argument parsing issues
- Function call fixes for better reliability

## [0.1.5] - 2025-08-28

### Added
- `list` command to display trash contents with details
- `recover <name>` command to restore specific files from trash
- `recover-all` command to restore all files from trash

### Changed
- Better trash content organization and display
- Improved documentation and tool information

### Fixed
- Function separation and code cleanup
- Logic improvements for ignore directory handling

## [0.1.4] - 2025-08-28

### Added
- Comprehensive installation instructions via cargo
- Enhanced README documentation

### Changed
- Updated project metadata and tool information
- Improved code organization and documentation

## [0.1.3] - 2025-08-27

### Added
- `list` command to view contents in trash directory
- Enhanced argument parsing and module organization

### Changed
- Updated argument handling structure
- Improved function organization

## [0.1.2] - 2025-08-27

### Added
- Empty directory handling with `-d, --dir` flag
- Enhanced error handling for home directory operations
- Logic for handling empty directories without moving them to trash

### Changed
- Improved directory handling logic
- Updated README with better documentation
- Enhanced CI/CD workflows for Rust projects

### Fixed
- Home directory error handling
- General error handling improvements

## [0.1.1] - 2025-08-26

### Added
- Basic trash functionality with file removal to system trash
- Recursive file removal with `-r, --recursive` flag
- Force removal without prompts using `-f, --force` flag
- Multiple file handling capability
- Cross-platform trash directory support
- Initial CI/CD setup with GitHub Actions
- Project documentation and README

### Changed
- Updated toolchain configuration
- Optimized CI workflow

### Fixed
- CI workflow typos and configuration issues

## [0.1.0] - 2025-08-25

### Added
- Initial project setup with Cargo configuration
- Core dependencies: chrono, clap, dirs, trash, walkdir
- Basic argument parsing structure with clap
- Foundation for trash-based file removal functionality
- Nix flake support for development environment
- MIT License
- Initial README and project documentation

[Unreleased]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.6...HEAD
[0.1.6]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/santoshxshrestha/rmxt/releases/tag/v0.1.0
