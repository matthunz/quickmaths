## Quickmaths
A collection of algorithims for generic mathematics in Rust.

[![crate](https://img.shields.io/crates/v/quickmaths.svg)](https://crates.io/crates/quickmaths)
[![documentation](https://docs.rs/quickmaths/badge.svg)](https://docs.rs/quickmaths)


## Features

This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```toml
[dependencies.quickmaths]
version = "0.1"
default-features = false
