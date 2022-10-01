# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Add an interactive mode for `track add` and `track remove` when no arguments
    are provided.

### Changed

* **BREAKING** Rename the state file from `<DATA_DIR>/track/data.toml` to
    `<DATA_DIR>/track/state.toml`.
* **BREAKING** Make the library APIs private.
* Improve usage error handling.
* Update the dependencies.

### Fixed

* [remove] Print the error message in red when a parcel was not tracked (#1).

## [0.1.1] - 2020-10-29

### Changed

* Switch the TLS stack from OpenSSL to rustls.

## [0.1.0] - 2020-08-03

* Initial release, featuring:
    * a quick-and-dirty client for the La Poste “Suivi v2” API,
    * a CLI with subcommands to:
        * initialise the configuration (i.e. the API key),
        * get tracking info for an indivitual parcel,
        * add, remove and list tracked parcel in a set,
        * get tracking info for all tracked parcels;
    * basic user-related error handling (lack of configuration, add/remove
      errors),
    * no machine-related error handling (including no error handling at all for
      the API).

[Unreleased]: https://github.com/ejpcmac/track/compare/main...develop
[0.1.1]: https://github.com/ejpcmac/track/compare/v0.1.0...0.1.1
[0.1.0]: https://github.com/ejpcmac/track/releases/tag/v0.1.0
