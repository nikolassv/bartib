# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Subcommand `search` to search the list of last activities for terms (thanks to [@Pyxels](https://github.com/Pyxels))
- Subcommand `status` to display the total duration of activities today, in the current week and in the current month (thanks to [@airenas](https://github.com/airenas))
- Option `--no-quotes` to `project` to suppres quotes in the projects list (thanks to [@defigli](https://github.com/defigli))

### Changed

- Update of libc:musl to support longarch64 (thanks to [@zhaixiaojuan](https://github.com/zhaixiaojuan))
- Improved general `--help` output (thanks to [@RossBarnie](https://github.com/RossBarnie))

## [1.1.0] - 2024-02-29

### Added

- This changelog file
- Option `--round` for `list` and `report` to round start and end times (thanks to [@berkes](https://github.com/berkes))
- Subcommand `change` to modify the currently running activity
- Subcommand `sanity` for several sanity checks on the bartib log (thanks to [@lukasdietrich](https://github.com/lukasdietrich))
- Compact mode `--compact` for `projects` - list currently running projects only
- GitHub Actions for tests (thanks to [@Ernaldis](https://github.com/Ernaldis))

### Changed

- `--project` filters for `list` and `report` now support wildcards (thanks to [@simonsan](https://github.com/simonsan))
- Display activities where start and end time is the same with a duration of "&lt;1m" ([issue #24](https://github.com/nikolassv/bartib/issues/24) - thanks to [@julianmatos97](https://github.com/julianmatos97))
- Do not display the number of days in durations - [issue #12](https://github.com/nikolassv/bartib/issues/12)
- Many minor adjustments to the documentation

## [1.0.1] - 2021-11-25

### Added

- Arguments for weekly reports (`current_week` and `last_week`)

### Fixed

- `projects` and `description` arguments for `start` are now explicitely required (thanks to [@camerondurham](https://github.com/camerondurham))

### Changed

- Many minor adjustments to the documentation

## [1.0.0] - 2021-11-15

### Added

- All the basic features: Tracking activities, reporting, listing etc.

[unreleased]: https://github.com/nikolassv/bartib/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/nikolassv/bartib/compare/v1.0.1...v1.1.0
[1.0.1]: https://github.com/nikolassv/bartib/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/nikolassv/bartib/releases/tag/v1.0.0
