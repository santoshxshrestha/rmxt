# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.13] - 2025-09-18

### Added

- **Flake package**: Added a Nix flake for packaging.

### Fixed

- **No args and path**: Handled the case of rmxt when running without passing the args and paths. Now, it displays the help message instead.
- Improved error handling: after reporting a missing file or directory, the program now skips further processing for that path.

### Changed

- update documentation about installation and README


## [0.1.12] - 2025-09-11

### Changed

- **Improved variable naming**: Updated variable names to be more descriptive and consistent across the codebase.
- **Changed the visibility**: Changed the visibility of the functions, methods, ... etc
- Update documentation

### Fixed

- **Conflicting trash items**: Updated logic to handle conflicts when recovering files with the same name.
  Now, if a file with the same name exists in the original location, the recovered file will be renamed with a timestamp suffix to avoid overwriting.

## [0.1.11] - 2025-09-07

### Changed

- **Renamed the ignore flag to permanent flag**: The `-i, --ignore` flag has been renamed to `-p, --permanent` to better indicate its functionality.
  It now permanently deletes files instead of moving them to the trash, aligning its behavior with the `rm` command.

## [0.1.10] - 2025-09-07

### Fixed

- **Critical ignore flag bug**: Fixed issue where `-i, --ignore` flag was not functioning correctly, causing errors while trying to ingore them while using `rmxt` commands

### Changed

- Imporved different cases handling in the ignore flag logic with match block

## [0.1.9] - 2025-09-01

### Fixed

- **Critical Time System Bug**: Fixed incorrect time comparison logic in `tidy` command that was comparing timestamps against duration values instead of proper cutoff times
- **Time Filtering Logic**: Corrected time filtering in `list -t` and `recover-all -t` commands to show items within the specified time range instead of showing newer items
- **Consistent Time Handling**: All time-based operations now use consistent comparison logic with proper cutoff time calculations
- **Borrow Checker Issues**: Resolved Rust ownership issues in tidy function by storing count before moving vectors

## [0.1.8] - 2025-09-01

### Added

- Installation and uninstallation scripts for easier setup and removal
- Comprehensive documentation in `docs/` directory
- Advanced usage documentation
- Install and usage guides

### Changed

- Updated README with new changes and better documentation structure
- Improved code linting and formatting
- Enhanced documentation organization

### Fixed

- Code linting issues
- Documentation formatting and structure

## [0.1.7] - 2025-08-31

### Added

- `tidy` command now has args to specify days
- `recover-all` command now has args to specify days from which content will be recovered
- `list` command now has args to filter files based on days
- Config for the formatter (tabled crate)
- `list` command now prints the content in trash in a table format
- Red colored error messages for better visibility

### Changed

- Updated README with new changes
- Improved code structure and organization
- `recover` command can now take multiple file names as args
- `purge` command can now take multiple file names as args
- Improved error handling and messages throughout the codebase

### Fixed

- Better error handling with Results and if let statements
- Minor bug fixes and improvements
- Function refactoring for better efficiency

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
- Core dependencies: chrono, clap, colored, tabled, trash
- Basic argument parsing structure with clap
- Foundation for trash-based file removal functionality
- Nix flake support for development environment
- MIT License
- Initial README and project documentation

[Unreleased]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.9...HEAD
[0.1.9]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.8...v0.1.9
[0.1.8]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/santoshxshrestha/rmxt/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/santoshxshrestha/rmxt/releases/tag/v0.1.0
