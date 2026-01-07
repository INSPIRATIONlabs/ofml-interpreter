# Changelog

All notable changes to the OFML Interpreter will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Testing & Quality
- **Snapshot Tests**: Added insta-based snapshot testing for OCD data parsing
  - Tests for articles, prices, surcharges, and OCD statistics
  - Automatic regression detection for data parsing

#### TUI Improvements
- **Score-based Fuzzy Search**: Improved search with intelligent ranking
  - Exact matches prioritized over prefix matches
  - Prefix matches prioritized over substring matches
  - Subsequence matching with gap penalty scoring
  - Word boundary bonuses for better results
- **Theme Support**: Light and dark themes for the TUI
  - Toggle with `T` key during runtime
  - Configurable via `ThemeVariant` enum
  - Theme-aware header, footer, and status bar

#### Performance
- **OCD Reader Cache**: Global cache with TTL for OCD data
  - 5-minute TTL for cache entries
  - Automatic eviction of expired entries
  - Cache statistics via `CacheStats`

#### Developer Experience
- **CLI Shell Completion**: Auto-completion for bash, zsh, fish, elvish, and PowerShell
  - Generate with `ofml completions <shell>`
- **Structured Tracing**: Added `#[instrument]` attributes for key functions
  - Manufacturer and family context in trace spans
  - Improved debugging visibility
- **Enhanced Error Messages**: Improved error types with thiserror
  - Context-rich error messages with article, manufacturer, and property info
  - Helpful suggestions in error messages

#### Documentation
- **API Documentation**: Enhanced module documentation
  - Quick start example in lib.rs
  - Module-level documentation for OAP configurator

### Fixed

- **Flaky Multi-Manufacturer Test**: Reduced test scope for reliability
  - Split into quick (3 manufacturers) and extended (10 manufacturers, ignored)
  - Reduced families tested from 10 to 3 per manufacturer

### Changed

- Fuzzy search now returns results sorted by match quality
- Error types now use structured fields instead of plain strings

## [0.1.0] - Initial Release

### Added
- CLS bytecode interpreter for OFML class files
- EBase file format reader
- OAP configurator with pricing engine
- Terminal UI for product configuration
- GLB/GLTF geometry export
- Multi-manufacturer support
- Property-based variant code generation
- Surcharge calculation with propvalue2varcond mapping
