//! Langyan provides a signal mechanism like Django's [signal](https://docs.djangoproject.com/en/3.1/topics/signals/).
//!
//! ## Example
//!
//! ```rust
//! use langyan::signal::{Signal, Receiver};
//!
//! fn after_save(filename: &str) {
//!     println!("filename is {}", filename);
//! }
//!
//! fn main() {
//!     let saved = Signal::new();
//!     let subscription = saved.connect(after_save);
//!
//!     // after saved file
//!     saved.send("hello.json");
//!
//!     // sometime you want disconnect
//!     drop(subscription)
//! }
//! ```

pub mod signal;
