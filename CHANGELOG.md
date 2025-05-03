# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/zh-TW/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-06-19

### Added

- Added RFC3339 UTC timestamp to each error entry
- Introduced `ErrorEntry` struct to replace plain `ErrorCode` log entries
- Integrated `chrono` crate for timestamp generation
- JSON output now includes precise occurrence time for each fault

### Changed

- `ErrorLog.history` now stores `Vec<ErrorEntry>` instead of `Vec<ErrorCode>`

---

## [0.2.0] - 2025-06-19

### Added

- Introduced `serde` and `serde_json` support
- Derived `Serialize` and `Deserialize` for `ErrorCode` and `ErrorLog`
- Added JSON export of error log to stdout using `serde_json::to_string_pretty`
- Added optional file output to `errors.json` in the project root
- Enables future extensibility (e.g., replay, diagnostics, remote telemetry)

### Changed

- None

### Fixed

- None

---

## [0.1.0] - 2025-06-19

### Added

- Initial version with basic ECU state machine (Off, Idle, Running, Overheated, Error)
- Simulated sensor input (temperature, RPM)
- Watchdog timer with timeout-based state transition
- Error log system tracking WatchdogTimeout and Overheated events
