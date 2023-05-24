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
//! > ```ignore
//! > #![feature(ip_in_core)]
//! > ```
//! >
//! > No external IP address provider is planned to be supported. If you want to
//! > use `fancy-ip` in `#[no_std]` context with the stable or beta toolchain:
//! > be patient.
//!
//! In order to use fancy-ip in `no_std` contexts, you must add this library in
//! your `Cargo.toml` disabling the default features:
//! ```toml
//! fancy-ip = { version = "0.1", default_features = false }
//! ```

#![crate_type = "proc-macro"]
extern crate proc_macro;

mod arg_parser;

use arg_parser::ArgParser;
use proc_macro_error::{abort, proc_macro_error};
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    str::FromStr,
};

use proc_macro::{TokenStream, Span};

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

fn generate_ip_stream(addr: &IpAddr) -> TokenStream {
    match addr {
        IpAddr::V4(ip) => {
            let ip_stream = generate_ipv4_stream(ip);

            format!("{OBJECT_PREFIX}::IpAddr::V4({ip_stream})")
                .parse()
                .unwrap()
        },
        IpAddr::V6(ip) => {
            let ip_stream = generate_ipv6_stream(ip);
            
            format!("{OBJECT_PREFIX}::IpAddr::V6({ip_stream})")
                .parse()
                .unwrap()
        }
    }
}

fn generate_ip_socket_stream(socket : &SocketAddr) -> TokenStream {
    match socket {
        SocketAddr::V4(socket) => {
            let socket_stream = generate_ipv4_socket_stream(socket);

            format!("{OBJECT_PREFIX}::SocketAddr::V4({socket_stream})")
                .parse()
                .unwrap()
        },
        SocketAddr::V6(socket) => {
            let socket_stream = generate_ipv6_socket_stream(socket);

            format!("{OBJECT_PREFIX}::SocketAddr::V6({socket_stream})")
                .parse()
                .unwrap()
        }
    }
}

fn report_error<T>(value : Result<T, arg_parser::Error>) -> T {
    match value {
        Ok(v) => v,
        Err(e) => {
            abort!(e.span(), "{}", e);
        }
    }
}

fn report_too_few_arguments_error(given : usize, expected : usize) -> ! {
    abort!(
        Span::call_site(),
        "Too few argument: Given {}, expected {}",
        given, expected
    );
}

fn report_too_many_arguments_error(span : Span, given : usize, expected : usize) -> ! {
    abort!(
        span,
        "Too many arguments: Given {}, expected {}",
        given, expected
    );
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

    let ip = if let Some((v, _)) = report_error(parser.next_string()) {
        Ipv4Addr::from_str(v.as_str()).unwrap()
    } else {
        report_too_few_arguments_error(0, 1);
    };

    if let Some(span) = report_error(parser.ignore_next()) {
        report_too_many_arguments_error(span, parser.count_arguments(), 1);
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

    let ip = if let Some((v, _)) = report_error(parser.next_string()) {
        Ipv6Addr::from_str(v.as_str()).unwrap()
    } else {
        report_too_few_arguments_error(0, 1);
    };
   
    if let Some(span) = report_error(parser.ignore_next()) {
        report_too_many_arguments_error(span, parser.count_arguments(), 1);
    }

    generate_ipv6_stream(&ip)
}

/// Generate an IP address from the standard textual representation (both 
/// support IPv4 and IPv6)
///
/// # Syntax
///
/// This macro works as a function which take only one argument: the string
/// representation of an IP address
///
/// # Example
///
/// ```
/// # use fancy_ip::ip;
///
/// assert_eq!(ip!("::1"), std::net::IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
/// assert_eq!(ip!("192.168.1.5"), std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 5)));
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn ip(item: TokenStream) -> TokenStream {
    let mut parser = ArgParser::from(item);

    let ip = if let Some((v, _)) = report_error(parser.next_string()) {
        IpAddr::from_str(v.as_str()).unwrap()
    } else {
        report_too_few_arguments_error(0, 1);
    };
   
    if let Some(span) = report_error(parser.ignore_next()) {
        report_too_many_arguments_error(span, parser.count_arguments(), 1);
    }

    generate_ip_stream(&ip)
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

    let socket = if let Some((v, _)) = report_error(parser.next_string()) {
        SocketAddrV4::from_str(v.as_str()).unwrap()
    } else {
        report_too_few_arguments_error(0, 1);
    };
   
    if let Some(span) = report_error(parser.ignore_next()) {
        report_too_many_arguments_error(span, parser.count_arguments(), 1);
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

    let mut socket = if let Some((v, span)) = report_error(parser.next_string()) {
        match SocketAddrV6::from_str(v.as_str()) {
            Ok(v) => v,
            Err(_) => {
                abort!(span, "The given address `{}` is not a valid IPv6 socket address", v);
            }
        }
    } else {
        report_too_few_arguments_error(0, 1);
    };

    if let Some((flow_info, _)) = report_error(parser.next_integer()) {
        socket.set_flowinfo(flow_info);
    }

    if let Some((scope_id, _)) = report_error(parser.next_integer()) {
        socket.set_scope_id(scope_id)
    }
   
    if let Some(span) = report_error(parser.ignore_next()) {
        report_too_many_arguments_error(span, parser.count_arguments(), 3);
    }

    generate_ipv6_socket_stream(&socket)
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
/// # use fancy_ip::socket;
///
/// assert_eq!(socket!("[::1]:3000"), std::net::SocketAddr::V6(std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 3000, 0, 0)));
/// assert_eq!(socket!("192.168.1.5:3000"), std::net::SocketAddr::V4(std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(192, 168, 1, 5), 3000)));
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn socket(item: TokenStream) -> TokenStream {
    let mut parser = ArgParser::from(item);

    let socket = if let Some((v, span)) = report_error(parser.next_string()) {
        match SocketAddr::from_str(v.as_str()) {
            Ok(v) => v,
            Err(_) => {
                abort!(span, "The given address `{}` is not a valid socket address", v);
            }
        }
    } else {
        report_too_few_arguments_error(0, 1);
    };
   
    if let Some(span) = report_error(parser.ignore_next()) {
        report_too_many_arguments_error(span, parser.count_arguments(), 1);
    }

    generate_ip_socket_stream(&socket)
}