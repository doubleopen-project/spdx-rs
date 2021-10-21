<!--
SPDX-FileCopyrightText: 2021 HH Partners
 
SPDX-License-Identifier: MIT
 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2021-10-21

### Changed

- **Breaking:** Refactor snippet byte range and line range.

### Removed

- **Breaking:** Moved the functionality for getting the SPDX License List to a utility crate. See
  [spdx-toolkit](https://github.com/doublopen-project/spdx-toolkit).
- **Breaking:** Moved the functionality for for creating graphs from the relationships to a utility
  crate. See [spdx-toolkit](https://github.com/doublopen-project/spdx-toolkit).

## [0.2.1] - 2021-10-14

### Fixed

- Accepts lowercase relationship types when parsing tag-value documents.

## [0.2.0] - 2021-10-13

### Changed

- Started following semantic versioning and keeping a changelog.

[unreleased]: https://github.com/doubleopen-project/spdx-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/doubleopen-project/spdx-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/doubleopen-project/spdx-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/doubleopen-project/spdx-rs/compare/v0.1.0...v0.2.0
