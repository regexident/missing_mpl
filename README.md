# missing_mpl

[![Build Status](http://img.shields.io/travis/regexident/missing_mpl.svg?style=flat-square)](https://travis-ci.org/regexident/missing_mpl)
[![Downloads](https://img.shields.io/crates/d/missing_mpl.svg?style=flat-square)](https://crates.io/crates/missing_mpl/)
[![Version](https://img.shields.io/crates/v/missing_mpl.svg?style=flat-square)](https://crates.io/crates/missing_mpl/)
[![License](https://img.shields.io/crates/l/missing_mpl.svg?style=flat-square)](https://crates.io/crates/missing_mpl/)

## Synopsis

A lint for watching over your project's MPL-2.0 licensed source files.

## Motivation

The MPL-2.0 license expects one to add a short license header to each and every file to be covered:

```
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
```

Working on a project it is all too easy to forget to add such header to newly created files.

The `missing_mpl` lint aims to help make sure all relevant source files include such a header.

## Getting Started

Add the most recent [version](https://crates.io/crates/missing_mpl) of `missing_mpl`
to your build-dependencies in your project's `Cargo.toml`.

Then add …

```rust
#![feature(plugin)]
#![plugin(missing_mpl)]

#![warn(missing_mpl)]
```

… to your crate's root file (e.g. `lib.rs`, `main.rs`).

Once that's done you're all set!

Now all you need to do is run `cargo build` and watch out for warnings:

> "warning: Missing MPL license header in source file."

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),
and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/regexident/missing_mpl/tags).

## Authors

* **Vincent Esche** – *Initial work* – [Regexident](https://github.com/Regexident)

See also the list of [contributors](https://github.com/regexident/missing_mpl/contributors) who participated in this project.

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
