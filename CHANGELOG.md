# Changelog

All notable changes to this project will be documented in this file.
This project follows the principles of [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- Support for reading logs from files or standard input (stdin)
- Streaming-friendly processing via Unix-style pipelines
- Strongly typed log severity levels
- Optional log level filtering via `--level`
- Strict validation mode to fail on malformed log entries (`--strict`)
- Structured JSON output for downstream consumption (`--json`)
- Date-based filtering using inclusive `--since` and `--until` flags
- Optional descending ordering of log entries via `--desc`
- Basic aggregation utilities such as counting log entries by severity

### Changed
- Refactored log ingestion to use generic `BufRead` abstractions
- Modularized core components into reader, parser, filters, and analyzer modules
- Improved ownership handling to avoid unnecessary cloning
- Simplified CLI argument parsing and execution flow

### Fixed
- Ownership-related issues when passing CLI arguments
- Parsing failures caused by malformed or partially structured log lines
