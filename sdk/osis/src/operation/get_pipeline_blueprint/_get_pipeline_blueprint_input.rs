// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPipelineBlueprintInput {
    /// <p>The name of the blueprint to retrieve.</p>
    pub blueprint_name: ::std::option::Option<::std::string::String>,
}
impl GetPipelineBlueprintInput {
    /// <p>The name of the blueprint to retrieve.</p>
    pub fn blueprint_name(&self) -> ::std::option::Option<&str> {
        self.blueprint_name.as_deref()
    }
}
impl GetPipelineBlueprintInput {
    /// Creates a new builder-style object to manufacture [`GetPipelineBlueprintInput`](crate::operation::get_pipeline_blueprint::GetPipelineBlueprintInput).
    pub fn builder() -> crate::operation::get_pipeline_blueprint::builders::GetPipelineBlueprintInputBuilder {
        crate::operation::get_pipeline_blueprint::builders::GetPipelineBlueprintInputBuilder::default()
    }
}

/// A builder for [`GetPipelineBlueprintInput`](crate::operation::get_pipeline_blueprint::GetPipelineBlueprintInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetPipelineBlueprintInputBuilder {
    pub(crate) blueprint_name: ::std::option::Option<::std::string::String>,
}
impl GetPipelineBlueprintInputBuilder {
    /// <p>The name of the blueprint to retrieve.</p>
    pub fn blueprint_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.blueprint_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the blueprint to retrieve.</p>
    pub fn set_blueprint_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.blueprint_name = input;
        self
    }
    /// <p>The name of the blueprint to retrieve.</p>
    pub fn get_blueprint_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.blueprint_name
    }
    /// Consumes the builder and constructs a [`GetPipelineBlueprintInput`](crate::operation::get_pipeline_blueprint::GetPipelineBlueprintInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_pipeline_blueprint::GetPipelineBlueprintInput, ::aws_smithy_http::operation::error::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_pipeline_blueprint::GetPipelineBlueprintInput {
            blueprint_name: self.blueprint_name,
        })
    }
}
