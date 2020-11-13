![Tracing — Structured, application-level diagnostics][splash]

[splash]: https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/splash.svg

# tracing-syslog

Support for logging [`tracing`][tracing] events natively to [syslog],
preserving structured information.

[![Crates.io][crates-badge]][crates-url]
[![Documentation (master)][docs-master-badge]][docs-master-url]
[![MIT licensed][mit-badge]][mit-url]
![maintenance status][maint-badge]

[crates-badge]: https://img.shields.io/crates/v/tracing-syslog.svg
[crates-url]: https://crates.io/crates/tracing-syslog
[docs-master-badge]: https://img.shields.io/badge/docs-master-blue
[docs-master-url]: https://tracing-rs.netlify.com/tracing_syslog
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[maint-badge]: https://img.shields.io/badge/maintenance-experimental-blue.svg

## Overview

[`tracing`] is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics. `tracing-syslog` provides a
[`tracing-subscriber::Layer`][layer] implementation for logging `tracing` spans
and events to [`syslog`][syslog], on Linux distributions.
 
*Compiler support: [requires `rustc` 1.42+][msrv]*

[msrv]: #supported-rust-versions
[`tracing`]: https://crates.io/crates/tracing
[layer]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/layer/trait.Layer.html
[syslog]: https://tools.ietf.org/html/rfc5424

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.42. The current Tracing version is not guaranteed to build on Rust
versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current stable
compiler version is 1.45, the minimum supported version will not be increased
past 1.42, three minor versions prior. Increasing the minimum supported compiler
version is not considered a semver breaking change as long as doing so complies
with this policy.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tracing by you, shall be licensed as MIT, without any additional
terms or conditions.
