#![warn(missing_docs)]
//! Bento SDK for Rust
//!
//! This crate provides a client for interacting with the Bento API.

/// A type alias for results used throughout the Bento SDK.
///
/// The `Result<T>` type is a convenient wrapper around `std::result::Result`,
/// with the error type fixed to `Error`. This is used to simplify function
/// signatures in the SDK.
pub type Result<T> = std::result::Result<T, Error>;

mod client;
mod config;
mod error;
mod types;

/// The broadcast module provides functionality for managing and interacting with broadcasts.
pub mod broadcast;

/// The email module offers utilities for handling email-related operations.
pub mod email;

/// The event module contains tools for managing events and event data.
pub mod event;

/// The experimental module includes features that are in a testing or beta phase.
pub mod experimental;

/// The field module provides structures and utilities for working with fields in Bento.
pub mod field;

/// The subscriber module enables management of subscribers within the Bento system.
pub mod subscriber;

/// The tag module provides functionality for working with tags.
pub mod tag;

/// The stats module includes tools for accessing and manipulating statistical data.
pub mod stats;

pub use client::Client;
pub use config::{Config, ConfigBuilder};
pub use error::Error;
pub use types::*;

/// Current version of the SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
