#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use http::StatusCode;
use hyper_body_utils::HttpBody;
use std::{future::Future, pin::Pin, sync::Arc};
use vetis::{errors::VetisError, Request, Response};

mod callback;
mod tests;

/// ASGI worker implementation
pub struct AsgiWorker {
    directory: String,
    target: String,
}

impl AsgiWorker {
    /// Create a new ASGI worker
    pub fn new(directory: String, target: String) -> AsgiWorker {
        AsgiWorker { directory, target }
    }

    /// Get the directory
    pub fn directory(&self) -> &String {
        &self.directory
    }

    /// Get the target
    pub fn target(&self) -> &String {
        &self.target
    }
}
