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
//! <fullname>AWS Resource Groups</fullname>
//!
//! <p>AWS Resource Groups lets you organize AWS resources such as Amazon EC2 instances, Amazon Relational Database Service
//! databases, and Amazon S3 buckets into groups using criteria that you define as tags. A
//! resource group is a collection of resources that match the resource types specified in a
//! query, and share one or more tags or portions of tags. You can create a group of
//! resources based on their roles in your cloud infrastructure, lifecycle stages, regions,
//! application layers, or virtually any criteria. Resource Groups enable you to automate management
//! tasks, such as those in AWS Systems Manager Automation documents, on tag-related resources in
//! AWS Systems Manager. Groups of tagged resources also let you quickly view a custom console in
//! AWS Systems Manager that shows AWS Config compliance and other monitoring data about member
//! resources.</p>
//! <p>To create a resource group, build a resource query, and specify tags that identify the
//! criteria that members of the group have in common. Tags are key-value pairs.</p>
//! <p>For more information about Resource Groups, see the <a href="https://docs.aws.amazon.com/ARG/latest/userguide/welcome.html">AWS Resource Groups User Guide</a>.</p>
//! <p>AWS Resource Groups uses a REST-compliant API that you can use to perform the following types of
//! operations.</p>
//! <ul>
//! <li>
//! <p>Create, Read, Update, and Delete (CRUD) operations on resource groups and
//! resource query entities</p>
//! </li>
//! <li>
//! <p>Applying, editing, and removing tags from resource groups</p>
//! </li>
//! <li>
//! <p>Resolving resource group member ARNs so they can be returned as search
//! results</p>
//! </li>
//! <li>
//! <p>Getting data about resources that are members of a group</p>
//! </li>
//! <li>
//! <p>Searching AWS resources based on a resource query</p>
//! </li>
//! </ul>
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

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("resourcegroups", PKG_VERSION);
pub use aws_credential_types::Credentials;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
#[doc(inline)]
pub use client::Client;
