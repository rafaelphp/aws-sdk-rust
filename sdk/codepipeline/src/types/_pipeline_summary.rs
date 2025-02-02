// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returns a summary of a pipeline.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PipelineSummary {
    /// <p>The name of the pipeline.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The version number of the pipeline.</p>
    pub version: ::std::option::Option<i32>,
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    pub created: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    pub updated: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PipelineSummary {
    /// <p>The name of the pipeline.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The version number of the pipeline.</p>
    pub fn version(&self) -> ::std::option::Option<i32> {
        self.version
    }
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    pub fn created(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created.as_ref()
    }
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    pub fn updated(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated.as_ref()
    }
}
impl PipelineSummary {
    /// Creates a new builder-style object to manufacture [`PipelineSummary`](crate::types::PipelineSummary).
    pub fn builder() -> crate::types::builders::PipelineSummaryBuilder {
        crate::types::builders::PipelineSummaryBuilder::default()
    }
}

/// A builder for [`PipelineSummary`](crate::types::PipelineSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PipelineSummaryBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<i32>,
    pub(crate) created: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl PipelineSummaryBuilder {
    /// <p>The name of the pipeline.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the pipeline.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the pipeline.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The version number of the pipeline.</p>
    pub fn version(mut self, input: i32) -> Self {
        self.version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version number of the pipeline.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i32>) -> Self {
        self.version = input;
        self
    }
    /// <p>The version number of the pipeline.</p>
    pub fn get_version(&self) -> &::std::option::Option<i32> {
        &self.version
    }
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    pub fn created(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    pub fn set_created(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created = input;
        self
    }
    /// <p>The date and time the pipeline was created, in timestamp format.</p>
    pub fn get_created(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created
    }
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    pub fn updated(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    pub fn set_updated(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated = input;
        self
    }
    /// <p>The date and time of the last update to the pipeline, in timestamp format.</p>
    pub fn get_updated(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated
    }
    /// Consumes the builder and constructs a [`PipelineSummary`](crate::types::PipelineSummary).
    pub fn build(self) -> crate::types::PipelineSummary {
        crate::types::PipelineSummary {
            name: self.name,
            version: self.version,
            created: self.created,
            updated: self.updated,
        }
    }
}
