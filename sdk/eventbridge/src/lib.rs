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
//! <p>Amazon EventBridge helps you to respond to state changes in your Amazon Web Services resources. When your
//! resources change state, they automatically send events to an event stream. You can create
//! rules that match selected events in the stream and route them to targets to take action. You
//! can also use rules to take action on a predetermined schedule. For example, you can configure
//! rules to:</p>
//! <ul>
//! <li>
//! <p>Automatically invoke an Lambda function to update DNS entries when an event
//! notifies you that Amazon EC2 instance enters the running state.</p>
//! </li>
//! <li>
//! <p>Direct specific API records from CloudTrail to an Amazon Kinesis data stream for
//! detailed analysis of potential security or availability risks.</p>
//! </li>
//! <li>
//! <p>Periodically invoke a built-in target to create a snapshot of an Amazon EBS
//! volume.</p>
//! </li>
//! </ul>
//! <p>For more information about the features of Amazon EventBridge, see the <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide">Amazon EventBridge User
//! Guide</a>.</p>
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

mod json_deser;

mod json_ser;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
                    pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::endpoint::Endpoint;
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("eventbridge", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]
pub use client::Client;
pub use aws_types::region::Region;
pub use aws_credential_types::Credentials;

