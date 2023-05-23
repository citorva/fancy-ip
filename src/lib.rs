//! Adds a fancy way to generate IP addresses and socket addresses from its
//! string representation
//!
//! This library aims to replace the use of `parse()` or `new()` functions for
//! initializing an IP address using a macro call. This approach allows the
//! emission of compile-time errors when an address is malformed and the use of
//! human-readable addresses in const contexts.
//!
//! # Using in `#[no_std]` contexts
//!
//! This library can be used in `#[no_std]` contexts by using the `core`
//! implementation of addresses instead of the `std` implementation.
//!
//! > ⚠️ Address in `core` is currently an unstable feature.
//! >
//! > In order to use this feature, you must use the nightly toolchain and
//! > enable the `ip_in_core` in `main.rs` or `lib.rs` as is:
//! > ```
//! > #![feature(ip_in_core)]
//! > ```
//! >
//! > No external IP address provider is planned to be supported. If you want to
//! > use `fancy-ip` in `#[no_std]` context with the stable or beta toolchain:
//! > be patient.
//!
//! In order to use fancy-ip in `no_std` contexts, you must add this library in
//! your `Cargo.toml` disabling the default features:
//! ```
//! fancy-ip = { version = "0.1", default_features = false }
//! ```

#![crate_type = "proc-macro"]
extern crate proc_macro;

mod arg_parser;

use arg_parser::ArgParser;
use proc_macro_error::{abort, proc_macro_error};
use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6},
    str::FromStr,
};

use proc_macro::TokenStream;

#[cfg(feature = "std")]
const OBJECT_PREFIX: &'static str = "std::net";

#[cfg(not(feature = "std"))]
const OBJECT_PREFIX: &'static str = "core::net";

fn generate_ipv4_stream(addr: &Ipv4Addr) -> TokenStream {
    let [a, b, c, d] = addr.octets();

    format!("{OBJECT_PREFIX}::Ipv4Addr::new({a}, {b}, {c}, {d})")
        .parse()
        .unwrap()
}

fn generate_ipv4_socket_stream(socket: &SocketAddrV4) -> TokenStream {
    let addr = socket.ip();
    let port = socket.port();

    let ip_stream = generate_ipv4_stream(addr);

    format!("{OBJECT_PREFIX}::SocketAddrV4::new({ip_stream},{port})")
        .parse()
        .unwrap()
}

fn generate_ipv6_stream(addr: &Ipv6Addr) -> TokenStream {
    let [a, b, c, d, e, f, g, h] = addr.segments();

    format!("{OBJECT_PREFIX}::Ipv6Addr::new({a}, {b}, {c}, {d}, {e}, {f}, {g}, {h})")
        .parse()
        .unwrap()
}

fn generate_ipv6_socket_stream(socket: &SocketAddrV6) -> TokenStream {
    let addr = socket.ip();
    let port = socket.port();
    let flow_info = socket.flowinfo();
    let scope_id = socket.scope_id();

    let ip_stream = generate_ipv6_stream(addr);

    format!("{OBJECT_PREFIX}::SocketAddrV6::new({ip_stream},{port},{flow_info},{scope_id})")
        .parse()
        .unwrap()
}

/// Generate an IPv4 address from the standard textual representation
///
/// # Syntax
///
/// This macro works as a function which take only one argument: the string
/// representation of an IP address
///
/// # Example
///
/// ```
/// # use fancy_ip::ipv4;
///
/// assert_eq!(ipv4!("192.168.1.5"), std::net::Ipv4Addr::new(192, 168, 1, 5));
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn ipv4(item: TokenStream) -> TokenStream {
    let mut parser = ArgParser::from(item);

    let ip = if let Some(v) = parser.next_string() {
        Ipv4Addr::from_str(v.as_str()).unwrap()
    } else {
        abort!(
            parser.last_span(),
            "The first argument must be a string giving the IPv4 address only"
        );
    };

    if !parser.is_end_reached() {
        abort!(
            parser.last_span(),
            "Too many argument given, only expected the IP address"
        );
    }

    generate_ipv4_stream(&ip)
}

/// Generate an IPv6 address from the standard textual representation
///
/// # Syntax
///
/// This macro works as a function which take only one argument: the string
/// representation of an IP address
///
/// # Example
///
/// ```
/// # use fancy_ip::ipv6;
///
/// assert_eq!(ipv6!("::1"), std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn ipv6(item: TokenStream) -> TokenStream {
    let mut parser = ArgParser::from(item);

    let ip = if let Some(v) = parser.next_string() {
        Ipv6Addr::from_str(v.as_str()).unwrap()
    } else {
        abort!(
            parser.last_span(),
            "The first argument must be a string giving the IPv6 address only"
        );
    };

    if !parser.is_end_reached() {
        abort!(
            parser.last_span(),
            "Too many argument given, only expected the IP address"
        );
    }

    generate_ipv6_stream(&ip)
}

/// Generates a socket address from its string representation
///
/// # Syntax
///
/// This macro works as a function which take only one argument: the string
/// representation of a socket address
///
/// # Example
///
/// ```
/// # use fancy_ip::socketv4;
///
/// assert_eq!(socketv4!("192.168.1.5:3000"), std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 1, 5), 3000));
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn socketv4(item: TokenStream) -> TokenStream {
    let mut parser = ArgParser::from(item);

    let socket = if let Some(v) = parser.next_string() {
        SocketAddrV4::from_str(v.as_str()).unwrap()
    } else {
        abort!(
            parser.last_span(),
            "The first argument must be a string giving the IPv4 address with optionnaly the port"
        );
    };

    if !parser.is_end_reached() {
        abort!(
            parser.last_span(),
            "Too many argument given, only expected the IP address"
        );
    }

    generate_ipv4_socket_stream(&socket)
}

/// Generates a socket address from its string representation
///
/// # Syntax
///
/// This macro works as a function which take only one argument: the string
/// representation of a socket address
///
/// # Example
///
/// ```
/// # use fancy_ip::socketv6;
///
/// assert_eq!(socketv6!("[::1]:3000"), std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 3000, 0, 0));
/// assert_eq!(socketv6!("[::]:8080", 58, 30), std::net::SocketAddrV6::new(std::net::Ipv6Addr::UNSPECIFIED, 8080, 58, 30));
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn socketv6(item: TokenStream) -> TokenStream {
    let mut parser = ArgParser::from(item);

    let mut socket = if let Some(v) = parser.next_string() {
        SocketAddrV6::from_str(v.as_str()).unwrap()
    } else {
        abort!(
            parser.last_span(),
            "The first argument must be a string giving the IPv6 address with optionnaly the port"
        );
    };

    if !parser.is_end_reached() {
        if let Some(flow_info) = parser.next_integer() {
            socket.set_flowinfo(flow_info);
        } else {
            abort!(
                parser.last_span(),
                "The second argument must be a 32 bit integer giving the IPv6 flow info"
            );
        }
    }

    if !parser.is_end_reached() {
        if let Some(scope_id) = parser.next_integer() {
            socket.set_scope_id(scope_id)
        } else {
            abort!(
                parser.last_span(),
                "The third argument must be a 32 bit integer giving the IPv6 scope id"
            );
        }
    }

    if !parser.is_end_reached() {
        abort!(
            parser.last_span(),
            "Too many argument given, only expected the IP address"
        );
    }

    generate_ipv6_socket_stream(&socket)
}
