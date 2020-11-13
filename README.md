# tracing-syslog
*The project is currently under development and is not ready for use in production.*

Support for logging [`tracing`][tracing] events natively to [`syslog`][syslog],
preserving structured information.

## Overview
 
*Compiler support: [requires `rustc` 1.42+][msrv]*

[msrv]: #supported-rust-versions
[tracing]: https://crates.io/crates/tracing
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

