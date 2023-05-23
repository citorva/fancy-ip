Elegantly initialize IP and socket addresses.

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/fancy-ip.svg
[crates-url]: https://crates.io/crates/fancy-ip
[docs-badge]: https://docs.rs/fancy-ip/badge.svg
[docs-url]: https://docs.rs/fancy-ip
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

## Overview

Initializing IP address in the source code is currently not easy. It exists many
methods in the standard library such as using the `parse` function on a string
describing an IP address, which needs the use of time at the initialization for
generating the address and the need to have the code executed to check if the IP
is valid. Another way is using the IP and socket address constructor which is
heavy.

## Usage

### In `std` contexts

This library can be used in place of `parse` and `new` calls. You can use this
library by simply add the following line in your `Cargo.toml` file:

```
fancy-ip = "1"
```

Then you have just to call `fancy_ip::ipv4!()` or `fancy_ip::ipv6!()` anywhere in
your source code. This library will automatically generate the constructor call
keeping your code clean and readable.

In the case of your IP address is malformed, the library will automatically
print you the error.

### In `no-std` contexts

This library can also support the `no-std` context by simply calling the `core`
equivalent in your code.

Note that IP addresses in core is currently in early stage and requires the
nightly toolchain using `ip_in_core` feature.