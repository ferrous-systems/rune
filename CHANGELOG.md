# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Change Log](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Moved the Web runtime into the `hotg-ai/rune` repo under the `bindings/web`
  directory and turned it into a NPM package

## [0.5.3] - 2021-08-11

### Fixed

- Fixed the algorithm used to locate internal dependencies when installed via
  crates.io

## [0.5.2] - 2021-08-11

### Added

- Users will no longer need to manually install nightly because a
  `rust-toolchain.toml` will be copied into the generated project
- If the `LIBRUNECORAL` environment variable is set or the `--librunecoral`
  flag is provided, `rune run` will use the specified shared library for
  hardware acceleration on TPU-enabled devices

### Changed

- The tensor dimensions specified in a Runefile now need to *exactly* match the
  dimensions expected by the model. Previously users would be allowed to pass
  something like `u8[192, 192]` to a model expecting `u8[1, 192, 192, 1]`.

## [0.4.0] - 2021-07-27

### Added

- Multiple instances of the same capability can now be provided to `rune run`
  from the command line (i.e. `rune run some.rune --image first.png --image second.jpeg`)
  ([#233](https://github.com/hotg-ai/rune/pull/233))
- Capabilities in a `Runefile.yml` can now specify which source they want to
  pull data from using a `source` argument ([#223](https://github.com/hotg-ai/rune/pull/223))
- Models can have multiple inputs and outputs ([#218](https://github.com/hotg-ai/rune/pull/218))
- Introduced a `rune-proc-blocks` crate containing everything you need to write
  a proc block ([#190](https://github.com/hotg-ai/rune/pull/190))
- We can now visualise pipelines containing multiple inputs and outputs
  ([#186](https://github.com/hotg-ai/rune/pull/186))
- Added a `rune inspect` sub-command which lets you extract information about
  a Rune ([#183](https://github.com/hotg-ai/rune/pull/183))
  - Information includes which `rune` binary was used to compile it and the
    pipeline it contains
  - The `.rune_graph` custom section contains a JSON blob with the pipeline
    graph
  - The `.rune_version` custom section contains a JSON representation of the
  - [`BuildInfo`][build-info] from the `rune-cli` crate
- The `IMAGE` capability built into `rune run` now supports resizing
  ([#170](https://github.com/hotg-ai/rune/issues/170))

### Changed

- **(breaking change)** All published crates are now prefixed with `hotg-` to
  avoid a naming collision with [the Rune programming language][rune-rs] on
  crates.io ([#236](https://github.com/hotg-ai/rune/pull/236))
- **(breaking change)** The YAML format now requires a `version: 1` property
  ([#194](https://github.com/hotg-ai/rune/pull/194))
- Proc blocks are now defined using a custom derive (`#[derive(ProcBlock)]`)
  which allows the `rune` command to inspect a crate and find information about
  the proc blocks it contains
- Renamed the `runic_types` crate to `rune-core` for consistency with our other
  `rune-XXX` crates

### Fixed

- Added a `Dockerfile` to the repository which will build the `rune`
  command-line tool from scratch and include all necessary dependencies
  ([#203](https://github.com/hotg-ai/rune/pull/203))

## [0.3.0] - TinyML Summit Release (2021-05-25)

### Added

- Proc blocks, capabilities, and outputs can now all have multiple inputs and
  outputs
- Introduced a more feature-rich YAML format (typically called `Runefile.yml`)
  for defining a Rune ([#140](https://github.com/hotg-ai/rune/pull/140))
- Created `rune_py`, a Python package which wraps a lot of the proc blocks
  in the Rune project so they can be used during training
- Created a docker image which wraps the `rune` command-line tool for use on
  Windows and MacOS
- Metadata about a Rune's pipeline is now included in the WebAssembly binary as
  a custom section
- New proc blocks for
  - Image normalization ([#160](https://github.com/hotg-ai/rune/pull/160))
  - Determining the top N confidence values and pairing them with their
    associated labels ([#151](https://github.com/hotg-ai/rune/pull/151))
- Allow users to use expressions as part of capability/proc block arguments
  (e.g. to do maths or use a constant defined elsewhere)
  ([#149](https://github.com/hotg-ai/rune/pull/149))

### Changed

- Moved the `model-info` sub-command out of our internal `xtask` tool and into
  the `rune` command-line tool itself (e.g. as `rune model-info ./sine.tflite`)
  so people can use it to discover information about TensorFlow Lite models
  ([#141](https://github.com/hotg-ai/rune/pull/141))

### Fixed

- Bug determining the Runefile's "current directory"
  ([#143](https://github.com/hotg-ai/rune/pull/143))
- Type names in generated Rust code can't contain hyphens
  ([#137](https://github.com/hotg-ai/rune/pull/137))

## [0.2.1] - 2021-03-21

<!-- next-url -->
[Unreleased]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.5.3...HEAD
[0.5.3]: https://github.com/assert-rs/predicates-rs/compare/hotg-rune-cli-v0.5.2...hotg-rune-cli-v0.5.3
[0.5.2]: https://github.com/assert-rs/predicates-rs/compare/v0.4.0...hotg-rune-cli-v0.5.2
[0.4.0]: https://github.com/hotg-ai/rune/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/hotg-ai/rune/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/hotg-ai/rune/compare/86763cdbb0...v0.2.1

[tinyml]: https://github.com/hotg-ai/rune/releases/tag/TinyMLSummity-RC1
[build-info]: https://docs.rs/build-info-common/0.0.23/build_info_common/struct.BuildInfo.html
[rune-rs]: https://rune-rs.github.io/
