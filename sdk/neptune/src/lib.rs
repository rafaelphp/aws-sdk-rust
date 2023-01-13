#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Neptune</fullname>
//! <p>Amazon Neptune is a fast, reliable, fully-managed graph database service that makes it
//! easy to build and run applications that work with highly connected datasets. The core of
//! Amazon Neptune is a purpose-built, high-performance graph database engine optimized for
//! storing billions of relationships and querying the graph with milliseconds latency. Amazon
//! Neptune supports popular graph models Property Graph and W3C's RDF, and their respective query
//! languages Apache TinkerPop Gremlin and SPARQL, allowing you to easily build queries that
//! efficiently navigate highly connected datasets. Neptune powers graph use cases such as
//! recommendation engines, fraud detection, knowledge graphs, drug discovery, and network
//! security.</p>
//!
//! <p>This interface reference for Amazon Neptune contains documentation for a programming or
//! command line interface you can use to manage Amazon Neptune. Note that Amazon Neptune is
//! asynchronous, which means that some interfaces might require techniques such as polling or
//! callback functions to determine when a command has been applied. In this reference, the
//! parameter descriptions indicate whether a command is applied immediately, on the next instance
//! reboot, or during the maintenance window. The reference structure is as follows, and we list
//! following some related topics from the user guide.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// Endpoint resolution functionality
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

/// Generated accessors for nested fields
mod lens;

mod query_ser;

mod xml_deser;

/// Endpoints standard library functions
mod endpoint_lib;

mod rest_xml_wrapped_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("neptune", PKG_VERSION);
pub use aws_credential_types::Credentials;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
#[doc(inline)]
pub use client::Client;
