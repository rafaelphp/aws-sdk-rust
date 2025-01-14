// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>When configuring application output, identifies an Amazon Kinesis stream as the destination. You provide the stream Amazon Resource Name (ARN) and also an IAM role ARN that Amazon Kinesis Analytics can use to write to the stream on your behalf.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KinesisStreamsOutput {
    /// <p>ARN of the destination Amazon Kinesis stream to write to.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
}
impl KinesisStreamsOutput {
    /// <p>ARN of the destination Amazon Kinesis stream to write to.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
}
impl KinesisStreamsOutput {
    /// Creates a new builder-style object to manufacture [`KinesisStreamsOutput`](crate::types::KinesisStreamsOutput).
    pub fn builder() -> crate::types::builders::KinesisStreamsOutputBuilder {
        crate::types::builders::KinesisStreamsOutputBuilder::default()
    }
}

/// A builder for [`KinesisStreamsOutput`](crate::types::KinesisStreamsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct KinesisStreamsOutputBuilder {
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
}
impl KinesisStreamsOutputBuilder {
    /// <p>ARN of the destination Amazon Kinesis stream to write to.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the destination Amazon Kinesis stream to write to.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>ARN of the destination Amazon Kinesis stream to write to.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// Consumes the builder and constructs a [`KinesisStreamsOutput`](crate::types::KinesisStreamsOutput).
    pub fn build(self) -> crate::types::KinesisStreamsOutput {
        crate::types::KinesisStreamsOutput {
            resource_arn: self.resource_arn,
            role_arn: self.role_arn,
        }
    }
}
